import { NoteEdge, NoteNode, LabelsMode } from '../../../src/types/graph.ts';
import { resolveNodeColor } from './colorResolver';

export interface CanvasRendererOptions {
  canvas: HTMLCanvasElement;
  initialLabelsMode?: LabelsMode;
  onNodeClick?: (node: NoteNode) => void;
  onNodeHover?: (node: NoteNode | null, x: number, y: number) => void;
  onNodeDragStart?: (node: NoteNode, x: number, y: number) => void;
  onNodeDrag?: (node: NoteNode, x: number, y: number) => void;
  onNodeDragEnd?: (node: NoteNode, x: number, y: number) => void;
  onNodeDoubleClick?: (node: NoteNode) => void;
}

export class CanvasRenderer {
  private canvas: HTMLCanvasElement;
  private ctx: CanvasRenderingContext2D;
  private nodes: NoteNode[] = [];
  private edges: NoteEdge[] = [];
  private dpr = window.devicePixelRatio || 1;
  private labelsMode: LabelsMode = 'hover';

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

  private hoveredNode: NoteNode | null = null;
  private animFrameId: number | null = null;
  private onNodeClick?: (node: NoteNode) => void;
  private onNodeHover?: (node: NoteNode | null, x: number, y: number) => void;
  private onNodeDragStart?: (node: NoteNode, x: number, y: number) => void;
  private onNodeDrag?: (node: NoteNode, x: number, y: number) => void;
  private onNodeDragEnd?: (node: NoteNode, x: number, y: number) => void;
  private onNodeDoubleClick?: (node: NoteNode) => void;

  constructor(options: CanvasRendererOptions) {
    this.canvas = options.canvas;
    this.ctx = options.canvas.getContext('2d')!;
    this.onNodeClick = options.onNodeClick;
    this.onNodeHover = options.onNodeHover;
    this.onNodeDragStart = options.onNodeDragStart;
    this.onNodeDrag = options.onNodeDrag;
    this.onNodeDragEnd = options.onNodeDragEnd;
    this.onNodeDoubleClick = options.onNodeDoubleClick;

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

  public resetZoom(): void {
    this.scale = 1;
    this.offsetX = 0;
    this.offsetY = 0;
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
      this.lastMouseX = e.clientX;
      this.lastMouseY = e.clientY;

      const hit = this.findNodeAt(e.clientX, e.clientY);
      if (hit) {
        this.draggedNode = hit;
        this.isNodeDragging = false;
        this.dragStartX = e.clientX;
        this.dragStartY = e.clientY;
        this.hasMoved = false;
      } else {
        this.isPanning = true;
      }
    });

    window.addEventListener('mousemove', (e) => {
      if (this.draggedNode) {
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
      } else if (this.isPanning) {
        const dx = e.clientX - this.lastMouseX;
        const dy = e.clientY - this.lastMouseY;
        this.offsetX += dx;
        this.offsetY += dy;
        this.lastMouseX = e.clientX;
        this.lastMouseY = e.clientY;
        this.canvas.style.cursor = 'grabbing';
      } else {
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
      if (this.draggedNode) {
        if (this.isNodeDragging && this.hasMoved) {
          const rect = this.canvas.getBoundingClientRect();
          const canvasX = e.clientX - rect.left;
          const canvasY = e.clientY - rect.top;
          const worldX = (canvasX - this.offsetX) / this.scale;
          const worldY = (canvasY - this.offsetY) / this.scale;
          this.onNodeDragEnd?.(this.draggedNode, worldX, worldY);
        } else {
          // Clean click without dragging -> open note
          this.onNodeClick?.(this.draggedNode);
        }
        this.draggedNode = null;
        this.isNodeDragging = false;
        this.hasMoved = false;
      }
      this.isPanning = false;
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
    });
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
    const base = 6;
    const hubExtra = node.hopDistance === 0 ? 6 : 0;
    const centralityExtra = (node.betweenness || 0) * 10;
    return base + hubExtra + centralityExtra;
  }

  private startLoop(): void {
    const render = () => {
      this.draw();
      this.animFrameId = requestAnimationFrame(render);
    };
    this.animFrameId = requestAnimationFrame(render);
  }

  public draw(): void {
    const width = this.canvas.clientWidth || 800;
    const height = this.canvas.clientHeight || 600;

    this.ctx.clearRect(0, 0, width, height);

    this.ctx.save();
    this.ctx.translate(this.offsetX, this.offsetY);
    this.ctx.scale(this.scale, this.scale);

    const nodeMap = new Map<string, NoteNode>();
    for (const node of this.nodes) {
      nodeMap.set(node.id, node);
    }

    // 1. Draw Edges
    for (const edge of this.edges) {
      const srcId = typeof edge.source === 'object' ? (edge.source as NoteNode).id : (edge.source as string);
      const tgtId = typeof edge.target === 'object' ? (edge.target as NoteNode).id : (edge.target as string);

      const srcNode = nodeMap.get(srcId);
      const tgtNode = tgtId ? nodeMap.get(tgtId) : undefined;

      if (!srcNode || srcNode.x === undefined || srcNode.y === undefined) continue;

      this.ctx.beginPath();
      this.ctx.moveTo(srcNode.x, srcNode.y);

      if (tgtNode && tgtNode.x !== undefined && tgtNode.y !== undefined) {
        this.ctx.lineTo(tgtNode.x, tgtNode.y);
      } else {
        // Floating broken edge pointing slightly outward
        this.ctx.lineTo(srcNode.x + 25, srcNode.y + 25);
      }

      if (edge.isBroken) {
        this.ctx.strokeStyle = '#EF4444'; // Dashed Red for Dead Links
        this.ctx.lineWidth = 1.5;
        this.ctx.setLineDash([4, 4]);
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
      if (node.x === undefined || node.y === undefined) continue;

      const r = this.getNodeRadius(node);
      const isCustomCluster =
        node.color !== '#6B7280' && node.color !== '#64748B' && node.color !== '#3B82F6';
      const fillColor = resolveNodeColor(node.color, node.hopDistance, isCustomCluster);

      this.ctx.beginPath();
      this.ctx.arc(node.x, node.y, r, 0, 2 * Math.PI);
      this.ctx.fillStyle = fillColor;
      this.ctx.fill();

      // Node border
      if (node.hopDistance === 0) {
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

      const isHub = node.hopDistance === 0;
      const isHovered = node === this.hoveredNode;
      const isProminent = (node.inDegree || 0) + (node.outDegree || 0) >= 4 || node.hopDistance === 1;

      // Label visibility:
      // In 'hover' mode (clean default): ONLY show label for hovered node and the central Hub!
      // In 'all' mode: show labels for all nodes (unless drastically zoomed out)
      const shouldDraw = this.labelsMode === 'all'
        ? (this.scale >= 0.35 || isHovered || isHub || isProminent)
        : (isHovered || isHub);

      if (!shouldDraw) continue;

      const r = this.getNodeRadius(node);
      const targetScreenPx = isHub ? 13 : (isHovered ? 12 : 11);
      // Scale-adaptive font size in world units so on-screen size remains steady and legible
      const worldFontPx = Math.max(4, Math.min(22, targetScreenPx / this.scale));
      const labelY = node.y + r + (4 / this.scale);

      this.ctx.font = `${isHub || isHovered ? 'bold ' : ''}${Math.round(worldFontPx)}px sans-serif`;

      // Dark background outline/stroke for maximum legibility across complex graphs
      this.ctx.lineWidth = 3.5 / this.scale;
      this.ctx.lineJoin = 'round';
      this.ctx.strokeStyle = 'rgba(15, 23, 42, 0.92)';
      this.ctx.strokeText(node.label, node.x, labelY);

      // Label foreground text
      if (isHub) {
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
