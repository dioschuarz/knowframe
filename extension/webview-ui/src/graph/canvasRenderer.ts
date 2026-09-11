import { NoteEdge, NoteNode } from '../../../src/types/graph';
import { resolveNodeColor } from './colorResolver';

export interface CanvasRendererOptions {
  canvas: HTMLCanvasElement;
  onNodeClick?: (node: NoteNode) => void;
  onNodeHover?: (node: NoteNode | null, x: number, y: number) => void;
}

export class CanvasRenderer {
  private canvas: HTMLCanvasElement;
  private ctx: CanvasRenderingContext2D;
  private nodes: NoteNode[] = [];
  private edges: NoteEdge[] = [];
  private dpr = window.devicePixelRatio || 1;

  // Zoom & Pan
  private scale = 1;
  private offsetX = 0;
  private offsetY = 0;
  private isDragging = false;
  private lastMouseX = 0;
  private lastMouseY = 0;

  private hoveredNode: NoteNode | null = null;
  private animFrameId: number | null = null;
  private onNodeClick?: (node: NoteNode) => void;
  private onNodeHover?: (node: NoteNode | null, x: number, y: number) => void;

  constructor(options: CanvasRendererOptions) {
    this.canvas = options.canvas;
    this.ctx = options.canvas.getContext('2d')!;
    this.onNodeClick = options.onNodeClick;
    this.onNodeHover = options.onNodeHover;

    this.setupDpr();
    this.setupEvents();
    this.startLoop();
  }

  public setData(nodes: NoteNode[], edges: NoteEdge[]): void {
    this.nodes = nodes;
    this.edges = edges;
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
      this.isDragging = true;
      this.lastMouseX = e.clientX;
      this.lastMouseY = e.clientY;

      const hit = this.findNodeAt(e.clientX, e.clientY);
      if (hit && this.onNodeClick) {
        this.onNodeClick(hit);
      }
    });

    window.addEventListener('mousemove', (e) => {
      if (this.isDragging) {
        const dx = e.clientX - this.lastMouseX;
        const dy = e.clientY - this.lastMouseY;
        this.offsetX += dx;
        this.offsetY += dy;
        this.lastMouseX = e.clientX;
        this.lastMouseY = e.clientY;
      } else {
        const hit = this.findNodeAt(e.clientX, e.clientY);
        if (hit !== this.hoveredNode) {
          this.hoveredNode = hit;
          this.canvas.style.cursor = hit ? 'pointer' : 'default';
        }
        if (this.onNodeHover) {
          this.onNodeHover(hit, e.clientX, e.clientY);
        }
      }
    });

    window.addEventListener('mouseup', () => {
      this.isDragging = false;
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

    // 1. Draw Edges
    for (const edge of this.edges) {
      const srcNode = typeof edge.source === 'object' ? edge.source : this.nodes.find((n) => n.id === edge.source);
      const tgtNode = typeof edge.target === 'object' ? edge.target : this.nodes.find((n) => n.id === edge.target);

      if (!srcNode || srcNode.x === undefined || srcNode.y === undefined) continue;

      this.ctx.beginPath();
      this.ctx.moveTo(srcNode.x, srcNode.y);

      if (tgtNode && tgtNode.x !== undefined && tgtNode.y !== undefined) {
        this.ctx.lineTo(tgtNode.x, tgtNode.y);
      } else {
        // Floating broken edge pointing slightly outward
        this.ctx.lineTo(srcNode.x + 30, srcNode.y + 30);
      }

      if (edge.isBroken) {
        this.ctx.strokeStyle = '#EF4444'; // Dashed Red for Dead Links
        this.ctx.lineWidth = 2;
        this.ctx.setLineDash([4, 4]);
      } else {
        this.ctx.strokeStyle = 'rgba(148, 163, 184, 0.4)';
        this.ctx.lineWidth = 1.5;
        this.ctx.setLineDash([]);
      }
      this.ctx.stroke();
    }

    this.ctx.setLineDash([]);

    // 2. Draw Nodes
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
      } else {
        this.ctx.lineWidth = 1;
        this.ctx.strokeStyle = 'rgba(255, 255, 255, 0.2)';
        this.ctx.stroke();
      }

      // Draw Label
      if (this.scale > 0.6 || node.hopDistance === 0 || node === this.hoveredNode) {
        this.ctx.fillStyle = node.hopDistance === 0 ? '#FFFFFF' : '#CBD5E1';
        this.ctx.font = node.hopDistance === 0 ? 'bold 12px sans-serif' : '11px sans-serif';
        this.ctx.textAlign = 'center';
        this.ctx.fillText(node.label, node.x, node.y + r + 14);
      }
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
