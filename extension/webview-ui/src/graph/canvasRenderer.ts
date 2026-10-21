import { NoteEdge, NoteNode, LabelsMode } from '../../../src/types/graph.ts';
import { resolveNodeColor } from './colorResolver';

export interface CanvasRendererOptions {
  canvas: HTMLCanvasElement;
  initialLabelsMode?: LabelsMode;
  onNodeClick?: (node: NoteNode) => void;
  onEdgeClick?: (edge: NoteEdge) => void;
  onNodeHover?: (node: NoteNode | null, x: number, y: number) => void;
  onNodeDragStart?: (node: NoteNode, x: number, y: number) => void;
  onNodeDrag?: (node: NoteNode, x: number, y: number) => void;
  onNodeDragEnd?: (node: NoteNode, x: number, y: number) => void;
  onNodeDoubleClick?: (node: NoteNode) => void;
  onViewTransformChange?: (transform: { scale: number; offsetX: number; offsetY: number }) => void;
}

export class CanvasRenderer {
  private canvas: HTMLCanvasElement;
  private ctx: CanvasRenderingContext2D;
  private nodes: NoteNode[] = [];
  private edges: NoteEdge[] = [];
  private dpr = window.devicePixelRatio || 1;
  private labelsMode: LabelsMode = 'hover';
  private matchedNodeIds: Set<string> | null = null;
  private showExcluded: boolean = false;
  private activeUnanchoredEdges: Set<string> = new Set();
  private showAllUnanchored: boolean = false;

  // Zoom & Pan
  private scale = 1;
  private offsetX = 0;
  private offsetY = 0;

  // Interaction State
  private isPanning = false;
  private draggedNode: NoteNode | null = null;
  private isNodeDragging = false;
  private dragStartX = 0;
  private dragStartY = 0;
  private hasMoved = false;
  private lastMouseX = 0;
  private lastMouseY = 0;
  private isCanvasMouseDown = false;

  private hoveredNode: NoteNode | null = null;
  private animFrameId: number | null = null;
  private onNodeClick?: (node: NoteNode) => void;
  private onEdgeClick?: (edge: NoteEdge) => void;
  private onNodeHover?: (node: NoteNode | null, x: number, y: number) => void;
  private onNodeDragStart?: (node: NoteNode, x: number, y: number) => void;
  private onNodeDrag?: (node: NoteNode, x: number, y: number) => void;
  private onNodeDragEnd?: (node: NoteNode, x: number, y: number) => void;
  private onNodeDoubleClick?: (node: NoteNode) => void;
  private onViewTransformChange?: (transform: { scale: number; offsetX: number; offsetY: number }) => void;

  constructor(options: CanvasRendererOptions) {
    this.canvas = options.canvas;
    this.ctx = options.canvas.getContext('2d')!;
    this.onNodeClick = options.onNodeClick;
    this.onEdgeClick = options.onEdgeClick;
    this.onNodeHover = options.onNodeHover;
    this.onNodeDragStart = options.onNodeDragStart;
    this.onNodeDrag = options.onNodeDrag;
    this.onNodeDragEnd = options.onNodeDragEnd;
    this.onNodeDoubleClick = options.onNodeDoubleClick;
    this.onViewTransformChange = options.onViewTransformChange;

    this.labelsMode = options.initialLabelsMode || 'hover';

    this.setupDpr();
    this.setupEvents();
    this.startLoop();
  }

  public setData(nodes: NoteNode[], edges: NoteEdge[]): void {
    this.nodes = nodes;
    this.edges = edges;
  }

  public setLabelsMode(mode: LabelsMode): void {
    this.labelsMode = mode;
  }

  public getLabelsMode(): LabelsMode {
    return this.labelsMode;
  }

  public setSearchFilter(matchedIds: Set<string> | null): void {
    this.matchedNodeIds = matchedIds;
  }

  public getSearchFilter(): Set<string> | null {
    return this.matchedNodeIds;
  }

  public setShowExcluded(show: boolean): void {
    this.showExcluded = show;
    this.draw();
  }

  public getShowExcluded(): boolean {
    return this.showExcluded;
  }

  public setActiveUnanchoredEdges(edgeKeys: Set<string>): void {
    this.activeUnanchoredEdges = edgeKeys;
    this.draw();
  }

  public setShowAllUnanchored(show: boolean): void {
    this.showAllUnanchored = show;
    this.draw();
  }

  public resetZoom(): void {
    this.scale = 1;
    this.offsetX = 0;
    this.offsetY = 0;
    this.onViewTransformChange?.({ scale: this.scale, offsetX: this.offsetX, offsetY: this.offsetY });
  }

  public getViewTransform(): { scale: number; offsetX: number; offsetY: number } {
    return { scale: this.scale, offsetX: this.offsetX, offsetY: this.offsetY };
  }

  public setViewTransform(transform: { scale: number; offsetX: number; offsetY: number }): void {
    if (transform && typeof transform.scale === 'number' && isFinite(transform.scale) && transform.scale > 0.05) {
      this.scale = Math.max(0.1, Math.min(5, transform.scale));
      this.offsetX = isFinite(transform.offsetX) ? transform.offsetX : 0;
      this.offsetY = isFinite(transform.offsetY) ? transform.offsetY : 0;
    }
  }

  public centerOnNode(nodeId: string): void {
    const node = this.nodes.find((n) => n.id === nodeId);
    if (node && node.x !== undefined && node.y !== undefined) {
      const width = this.canvas.clientWidth || 800;
      const height = this.canvas.clientHeight || 600;
      this.offsetX = width / 2 - node.x * this.scale;
      this.offsetY = height / 2 - node.y * this.scale;
      this.draw();
      this.onViewTransformChange?.({ scale: this.scale, offsetX: this.offsetX, offsetY: this.offsetY });
    }
  }

  public resize(width: number, height: number): void {
    this.canvas.width = width * this.dpr;
    this.canvas.height = height * this.dpr;
    this.canvas.style.width = `${width}px`;
    this.canvas.style.height = `${height}px`;
    this.ctx.setTransform(1, 0, 0, 1, 0, 0);
    this.ctx.scale(this.dpr, this.dpr);
  }

  private setupDpr(): void {
    const width = this.canvas.clientWidth || 800;
    const height = this.canvas.clientHeight || 600;
    this.resize(width, height);
  }

  private setupEvents(): void {
    this.canvas.addEventListener('mousedown', (e) => {
      this.isCanvasMouseDown = true;
      this.lastMouseX = e.clientX;
      this.lastMouseY = e.clientY;
      this.dragStartX = e.clientX;
      this.dragStartY = e.clientY;
      this.hasMoved = false;

      const hit = this.findNodeAt(e.clientX, e.clientY);
      if (hit) {
        this.draggedNode = hit;
        this.isNodeDragging = false;
      } else {
        this.isPanning = true;
      }
    });

    window.addEventListener('mousemove', (e) => {
      if (this.draggedNode && this.isCanvasMouseDown) {
        const dist = Math.hypot(e.clientX - this.dragStartX, e.clientY - this.dragStartY);
        if (dist > 3) {
          const rect = this.canvas.getBoundingClientRect();
          const canvasX = e.clientX - rect.left;
          const canvasY = e.clientY - rect.top;
          const worldX = (canvasX - this.offsetX) / this.scale;
          const worldY = (canvasY - this.offsetY) / this.scale;

          if (!this.isNodeDragging) {
            this.isNodeDragging = true;
            this.canvas.style.cursor = 'grabbing';
            this.onNodeDragStart?.(this.draggedNode, worldX, worldY);
          }
          this.hasMoved = true;
          this.onNodeDrag?.(this.draggedNode, worldX, worldY);
        }
      } else if (this.isPanning && this.isCanvasMouseDown) {
        const dx = e.clientX - this.lastMouseX;
        const dy = e.clientY - this.lastMouseY;
        if (Math.hypot(dx, dy) > 2) {
          this.hasMoved = true;
        }
        this.offsetX += dx;
        this.offsetY += dy;
        this.lastMouseX = e.clientX;
        this.lastMouseY = e.clientY;
        this.canvas.style.cursor = 'grabbing';
      } else {
        // Only trigger node hover when hovering directly over the canvas element
        if (e.target !== this.canvas) {
          if (this.hoveredNode) {
            this.hoveredNode = null;
          }
          this.onNodeHover?.(null, e.clientX, e.clientY);
          return;
        }
        const hit = this.findNodeAt(e.clientX, e.clientY);
        if (hit !== this.hoveredNode) {
          this.hoveredNode = hit;
          this.canvas.style.cursor = hit ? 'grab' : 'default';
        }
        if (this.onNodeHover) {
          this.onNodeHover(hit, e.clientX, e.clientY);
        }
      }
    });

    window.addEventListener('mouseup', (e) => {
      if (!this.isCanvasMouseDown) {
        return;
      }
      this.isCanvasMouseDown = false;

      if (this.draggedNode) {
        if (this.isNodeDragging && this.hasMoved) {
          const rect = this.canvas.getBoundingClientRect();
          const canvasX = e.clientX - rect.left;
          const canvasY = e.clientY - rect.top;
          const worldX = (canvasX - this.offsetX) / this.scale;
          const worldY = (canvasY - this.offsetY) / this.scale;
          this.onNodeDragEnd?.(this.draggedNode, worldX, worldY);
        } else {
          // Clean click on node without dragging -> open note
          this.onNodeClick?.(this.draggedNode);
        }
        this.draggedNode = null;
        this.isNodeDragging = false;
        this.hasMoved = false;
      } else if (!this.hasMoved && this.isPanning) {
        // Clean click on canvas background without panning/dragging
        const hitEdge = this.findEdgeAt(e.clientX, e.clientY);
        if (hitEdge) {
          this.onEdgeClick?.(hitEdge);
        }
      }
      const wasPanning = this.isPanning;
      this.isPanning = false;
      this.hasMoved = false;
      if (wasPanning) {
        this.onViewTransformChange?.({ scale: this.scale, offsetX: this.offsetX, offsetY: this.offsetY });
      }
      const hit = this.findNodeAt(e.clientX, e.clientY);
      this.canvas.style.cursor = hit ? 'grab' : 'default';
    });

    this.canvas.addEventListener('dblclick', (e) => {
      const hit = this.findNodeAt(e.clientX, e.clientY);
      if (hit && this.onNodeDoubleClick) {
        this.onNodeDoubleClick(hit);
      }
    });

    this.canvas.addEventListener('wheel', (e) => {
      e.preventDefault();
      const zoomFactor = e.deltaY < 0 ? 1.1 : 0.9;
      const newScale = Math.max(0.1, Math.min(5, this.scale * zoomFactor));

      const rect = this.canvas.getBoundingClientRect();
      const mouseX = e.clientX - rect.left;
      const mouseY = e.clientY - rect.top;

      this.offsetX -= (mouseX - this.offsetX) * (newScale / this.scale - 1);
      this.offsetY -= (mouseY - this.offsetY) * (newScale / this.scale - 1);
      this.scale = newScale;
      this.onViewTransformChange?.({ scale: this.scale, offsetX: this.offsetX, offsetY: this.offsetY });
    });
  }

  public findEdgeAt(clientX: number, clientY: number): NoteEdge | null {
    const rect = this.canvas.getBoundingClientRect();
    const worldX = (clientX - rect.left - this.offsetX) / this.scale;
    const worldY = (clientY - rect.top - this.offsetY) / this.scale;

    const nodeMap = new Map<string, NoteNode>();
    for (const node of this.nodes) {
      nodeMap.set(node.id, node);
    }

    const threshold = 8 / this.scale;
    const sortedEdges = [...this.edges].sort((a, b) => (b.isBroken ? 1 : 0) - (a.isBroken ? 1 : 0));

    for (const edge of sortedEdges) {
      const srcId = typeof edge.source === 'object' ? (edge.source as NoteNode).id : (edge.source as string);
      const tgtId = typeof edge.target === 'object' ? (edge.target as NoteNode).id : (edge.target as string);
      const srcNode = nodeMap.get(srcId);
      if (!srcNode || srcNode.x === undefined || srcNode.y === undefined) continue;

      const tgtNode = tgtId ? nodeMap.get(tgtId) : undefined;
      if (!this.showExcluded && (srcNode.status === 'excluded' || tgtNode?.status === 'excluded')) continue;

      const x1 = srcNode.x;
      const y1 = srcNode.y;
      const x2 = tgtNode && tgtNode.x !== undefined ? tgtNode.x : x1 + 25;
      const y2 = tgtNode && tgtNode.y !== undefined ? tgtNode.y : y1 + 25;

      const dx = x2 - x1;
      const dy = y2 - y1;
      const lenSq = dx * dx + dy * dy;
      let dist = 0;
      if (lenSq === 0) {
        dist = Math.hypot(worldX - x1, worldY - y1);
      } else {
        let t = ((worldX - x1) * dx + (worldY - y1) * dy) / lenSq;
        t = Math.max(0, Math.min(1, t));
        dist = Math.hypot(worldX - (x1 + t * dx), worldY - (y1 + t * dy));
      }

      if (dist <= threshold) {
        return edge;
      }
    }
    return null;
  }

  private findNodeAt(clientX: number, clientY: number): NoteNode | null {
    const rect = this.canvas.getBoundingClientRect();
    const canvasX = clientX - rect.left;
    const canvasY = clientY - rect.top;

    // Transform to world coordinates
    const worldX = (canvasX - this.offsetX) / this.scale;
    const worldY = (canvasY - this.offsetY) / this.scale;

    for (let i = this.nodes.length - 1; i >= 0; i--) {
      const node = this.nodes[i];
      if (node.x === undefined || node.y === undefined) continue;
      if (node.status === 'excluded' && !this.showExcluded) continue;

      const r = this.getNodeRadius(node);
      const dx = worldX - node.x;
      const dy = worldY - node.y;
      if (dx * dx + dy * dy <= r * r) {
        return node;
      }
    }
    return null;
  }

  private getNodeRadius(node: NoteNode): number {
    if (node.hopDistance === 0) return 14;
    if (node.hopDistance === 1) return 9;
    if (node.hopDistance === 2) return 6;
    if (node.hopDistance >= 3) return 4;
    return 4;
  }

  private startLoop(): void {
    const render = () => {
      this.draw();
      this.animFrameId = requestAnimationFrame(render);
    };
    this.animFrameId = requestAnimationFrame(render);
  }

  public draw(): void {
    const width = this.canvas.clientWidth || window.innerWidth || 800;
    const height = this.canvas.clientHeight || window.innerHeight || 600;

    if (this.canvas.width !== width * this.dpr || this.canvas.height !== height * this.dpr) {
      this.resize(width, height);
    }

    this.ctx.clearRect(0, 0, width, height);

    this.ctx.save();
    this.ctx.translate(this.offsetX, this.offsetY);
    this.ctx.scale(this.scale, this.scale);

    const nodeMap = new Map<string, NoteNode>();
    for (const node of this.nodes) {
      nodeMap.set(node.id, node);
    }

    const isSearchActive = this.matchedNodeIds !== null;

    // 1. Draw Edges
    for (const edge of this.edges) {
      const srcId = typeof edge.source === 'object' ? (edge.source as NoteNode).id : (edge.source as string);
      const tgtId = typeof edge.target === 'object' ? (edge.target as NoteNode).id : (edge.target as string);

      const srcNode = nodeMap.get(srcId);
      const tgtNode = tgtId ? nodeMap.get(tgtId) : undefined;

      if (!srcNode || typeof srcNode.x !== 'number' || !isFinite(srcNode.x) || typeof srcNode.y !== 'number' || !isFinite(srcNode.y)) {
        continue;
      }
      if (!this.showExcluded && (srcNode.status === 'excluded' || tgtNode?.status === 'excluded')) continue;

      const srcMatched = !isSearchActive || this.matchedNodeIds!.has(srcId);
      const tgtMatched = !isSearchActive || this.matchedNodeIds!.has(tgtId);

      this.ctx.beginPath();
      this.ctx.moveTo(srcNode.x, srcNode.y);

      if (tgtNode && typeof tgtNode.x === 'number' && isFinite(tgtNode.x) && typeof tgtNode.y === 'number' && isFinite(tgtNode.y)) {
        this.ctx.lineTo(tgtNode.x, tgtNode.y);
      } else {
        // Floating broken edge pointing slightly outward
        this.ctx.lineTo(srcNode.x + 25, srcNode.y + 25);
      }

      if (srcNode.status === 'excluded' || tgtNode?.status === 'excluded') {
        this.ctx.strokeStyle = 'rgba(148, 163, 184, 0.15)';
        this.ctx.lineWidth = 0.8;
        this.ctx.setLineDash([2, 4]);
      } else if (isSearchActive && (!srcMatched || !tgtMatched)) {
        // Dim edge if either node is outside search results
        this.ctx.strokeStyle = 'rgba(148, 163, 184, 0.05)';
        this.ctx.lineWidth = 0.8;
        this.ctx.setLineDash([]);
      } else if (edge.isBroken) {
        this.ctx.strokeStyle = '#EF4444'; // Dashed Red for Dead Links
        this.ctx.lineWidth = isSearchActive ? 2 : 1.5;
        this.ctx.setLineDash([4, 4]);
      } else if (!edge.anchor && !edge.target_fragment && tgtNode && (tgtNode.fragments?.length || 0) > 1 && (this.showAllUnanchored || this.activeUnanchoredEdges.has(`${edge.source}->${edge.target}`) || this.activeUnanchoredEdges.has(`${srcNode?.id}->${tgtNode?.id}`))) {
        // Amber dash for unanchored references to multi-fragment documents that are active findings (or when showAllUnanchored is enabled)
        this.ctx.strokeStyle = '#F59E0B'; // Amber / Orange for Missing Section Fragment
        this.ctx.lineWidth = isSearchActive ? 1.8 : 1.4;
        this.ctx.setLineDash([3, 3]);
      } else if (isSearchActive && srcMatched && tgtMatched) {
        // Bright active edge between search hits
        this.ctx.strokeStyle = 'rgba(56, 189, 248, 0.75)';
        this.ctx.lineWidth = 1.8;
        this.ctx.setLineDash([]);
      } else {
        this.ctx.strokeStyle = 'rgba(148, 163, 184, 0.35)';
        this.ctx.lineWidth = 1.2;
        this.ctx.setLineDash([]);
      }
      this.ctx.stroke();
    }

    this.ctx.setLineDash([]);

    // 2. Draw Nodes (Pass 1: circles & rings)
    for (const node of this.nodes) {
      if (!node || typeof node.x !== 'number' || !isFinite(node.x) || typeof node.y !== 'number' || !isFinite(node.y)) {
        continue;
      }

      const r = this.getNodeRadius(node);

      if (node.status === 'excluded') {
        if (!this.showExcluded) continue;
        this.ctx.beginPath();
        this.ctx.arc(node.x, node.y, r, 0, 2 * Math.PI);
        this.ctx.fillStyle = 'rgba(148, 163, 184, 0.05)';
        this.ctx.fill();
        this.ctx.lineWidth = 1;
        this.ctx.strokeStyle = 'rgba(148, 163, 184, 0.35)';
        this.ctx.setLineDash([2, 3]);
        this.ctx.stroke();
        this.ctx.setLineDash([]);
        continue;
      }

      const isMatched = !isSearchActive || this.matchedNodeIds!.has(node.id);

      if (isSearchActive && !isMatched) {
        // Faded / Dimmed node: low opacity so search hits stand out cleanly
        this.ctx.beginPath();
        this.ctx.arc(node.x, node.y, r, 0, 2 * Math.PI);
        this.ctx.fillStyle = 'rgba(100, 116, 139, 0.08)';
        this.ctx.fill();

        this.ctx.lineWidth = 0.8;
        this.ctx.strokeStyle = 'rgba(255, 255, 255, 0.04)';
        this.ctx.stroke();
        continue;
      }

      // Matched node (or normal mode)
      const isCustomCluster =
        node.color !== '#6B7280' && node.color !== '#64748B' && node.color !== '#475569';
      const fillColor = isSearchActive
        ? resolveNodeColor(node.color, Math.min(node.hopDistance, 1), isCustomCluster)
        : resolveNodeColor(node.color, node.hopDistance, isCustomCluster);

      this.ctx.beginPath();
      this.ctx.arc(node.x, node.y, r, 0, 2 * Math.PI);
      this.ctx.fillStyle = fillColor;
      this.ctx.fill();

      // Node border
      if (isSearchActive) {
        // Vivid cyan highlight ring for search matches
        this.ctx.lineWidth = 2.5;
        this.ctx.strokeStyle = '#38BDF8';
        this.ctx.stroke();

        // Outer glow halo ring
        this.ctx.beginPath();
        this.ctx.arc(node.x, node.y, r + 3, 0, 2 * Math.PI);
        this.ctx.lineWidth = 1.5;
        this.ctx.strokeStyle = 'rgba(56, 189, 248, 0.45)';
        this.ctx.stroke();
      } else if (node.hopDistance === 0) {
        this.ctx.lineWidth = 3;
        this.ctx.strokeStyle = '#3B82F6';
        this.ctx.stroke();
      } else if (node.brokenLinks > 0) {
        this.ctx.lineWidth = 2;
        this.ctx.strokeStyle = '#EF4444';
        this.ctx.stroke();
      } else if (node.fx !== null && node.fx !== undefined) {
        this.ctx.lineWidth = 2.5;
        this.ctx.strokeStyle = '#F59E0B';
        this.ctx.stroke();
      } else {
        this.ctx.lineWidth = 1;
        this.ctx.strokeStyle = 'rgba(255, 255, 255, 0.2)';
        this.ctx.stroke();
      }

      // Small amber pin indicator for manually positioned nodes
      if (node.fx !== null && node.fx !== undefined) {
        this.ctx.beginPath();
        this.ctx.arc(node.x + r * 0.7, node.y - r * 0.7, Math.max(2, 3 / this.scale), 0, 2 * Math.PI);
        this.ctx.fillStyle = '#F59E0B';
        this.ctx.fill();
      }
    }

    // 3. Draw Labels (Pass 2: on top of all circles to prevent clipping)
    this.ctx.textAlign = 'center';
    this.ctx.textBaseline = 'top';

    for (const node of this.nodes) {
      if (node.x === undefined || node.y === undefined) continue;
      if (node.status === 'excluded' && !this.showExcluded) continue;

      const isMatched = !isSearchActive || this.matchedNodeIds!.has(node.id);
      const isHub = node.hopDistance === 0;
      const isHovered = node === this.hoveredNode;
      const isProminent = (node.inDegree || 0) + (node.outDegree || 0) >= 4 || node.hopDistance === 1;

      // Label visibility:
      // When search is active, show labels ONLY for matching nodes (and hovered node)
      // When search is inactive, follow normal labelsMode ('hover' or 'all')
      const shouldDraw = isSearchActive
        ? (isMatched || isHovered)
        : (this.labelsMode === 'all'
            ? (this.scale >= 0.35 || isHovered || isHub || isProminent)
            : (isHovered || isHub));

      if (!shouldDraw) continue;

      const r = this.getNodeRadius(node);
      const targetScreenPx = (isSearchActive && isMatched) ? 12.5 : (isHub ? 13 : (isHovered ? 12 : 11));
      // Scale-adaptive font size in world units so on-screen size remains steady and legible
      const worldFontPx = Math.max(4, Math.min(22, targetScreenPx / this.scale));
      const labelY = node.y + r + (4 / this.scale);

      this.ctx.font = `${isHub || isHovered || (isSearchActive && isMatched) ? 'bold ' : ''}${Math.round(worldFontPx)}px sans-serif`;

      // Dark background outline/stroke for maximum legibility across complex graphs
      this.ctx.lineWidth = 3.5 / this.scale;
      this.ctx.lineJoin = 'round';
      this.ctx.strokeStyle = 'rgba(15, 23, 42, 0.92)';
      this.ctx.strokeText(node.label, node.x, labelY);

      // Label foreground text
      if (node.status === 'excluded') {
        this.ctx.fillStyle = 'rgba(148, 163, 184, 0.45)';
      } else if (isSearchActive && isMatched) {
        this.ctx.fillStyle = '#38BDF8';
      } else if (isHub) {
        this.ctx.fillStyle = '#60A5FA';
      } else if (isHovered) {
        this.ctx.fillStyle = '#38BDF8';
      } else {
        this.ctx.fillStyle = '#E2E8F0';
      }
      this.ctx.fillText(node.label, node.x, labelY);
    }

    this.ctx.restore();
  }

  public dispose(): void {
    if (this.animFrameId) {
      cancelAnimationFrame(this.animFrameId);
      this.animFrameId = null;
    }
  }
}
