import * as d3 from 'd3-force';
import type { NoteNode, NoteEdge, PhysicsParameters } from '../../../src/types/graph.ts';
import { DEFAULT_PHYSICS_PARAMS } from '../../../src/types/graph.ts';

export interface PhysicsSimulation {
  simulation: d3.Simulation<NoteNode, NoteEdge>;
  updateData: (nodes: NoteNode[], edges: NoteEdge[]) => void;
  updateParameters: (params: Partial<PhysicsParameters>) => void;
  getParameters: () => PhysicsParameters;
  resize: (width: number, height: number) => void;
  onDragStart: (node: NoteNode) => void;
  onDrag: (node: NoteNode, x: number, y: number) => void;
  onDragEnd: (node: NoteNode, x: number, y: number, pin?: boolean) => void;
  unpinNode: (node: NoteNode) => void;
  unpinAll: () => void;
  getPinnedCount: () => number;
  resetDisposition: () => void;
  restart: (alpha?: number) => void;
  stop: () => void;
}

export function createPhysicsSimulation(
  width: number,
  height: number,
  onTick: () => void,
  initialParams?: Partial<PhysicsParameters>,
): PhysicsSimulation {
  let currentWidth = width;
  let currentHeight = height;
  let params: PhysicsParameters = { ...DEFAULT_PHYSICS_PARAMS, ...initialParams };
  let currentNodes: NoteNode[] = [];
  let currentEdges: NoteEdge[] = [];

  const manyBody = d3.forceManyBody<NoteNode>()
    .strength(-params.repulsion)
    .distanceMax(700);

  const forceX = d3.forceX<NoteNode>(width / 2).strength(params.centerGravity);
  const forceY = d3.forceY<NoteNode>(height / 2).strength(params.centerGravity);
  const center = d3.forceCenter<NoteNode>(width / 2, height / 2);

  const collide = d3.forceCollide<NoteNode>()
    .radius((d) => params.collisionRadius + (d.betweenness || 0) * 12 + (d.hopDistance === 0 ? 10 : 0))
    .iterations(2);

  const link = d3.forceLink<NoteNode, NoteEdge>()
    .id((d) => d.id)
    .distance(params.linkDistance);

  const simulation = d3
    .forceSimulation<NoteNode, NoteEdge>()
    .force('charge', manyBody)
    .force('gravityX', forceX)
    .force('gravityY', forceY)
    .force('center', center)
    .force('collision', collide)
    .force('link', link)
    .on('tick', onTick);

  function applyEdges(): void {
    const nodeMap = new Set(currentNodes.map((n) => n.id));
    const validEdges = currentEdges
      .filter((e) => {
        const srcId = typeof e.source === 'object' ? (e.source as NoteNode).id : (e.source as string);
        const tgtId = typeof e.target === 'object' ? (e.target as NoteNode).id : (e.target as string);
        return !e.isBroken && nodeMap.has(srcId) && nodeMap.has(tgtId);
      })
      .map((e) => ({
        ...e,
        source: typeof e.source === 'object' ? (e.source as NoteNode).id : (e.source as string),
        target: typeof e.target === 'object' ? (e.target as NoteNode).id : (e.target as string),
      }));
    link.links(validEdges);
  }

  return {
    simulation,
    updateData: (nodes: NoteNode[], edges: NoteEdge[]) => {
      // Preserve node positions and pinned coordinates across updates
      const existingMap = new Map<string, { x?: number; y?: number; vx?: number; vy?: number; fx?: number | null; fy?: number | null }>();
      for (const n of currentNodes) {
        existingMap.set(n.id, { x: n.x, y: n.y, vx: n.vx, vy: n.vy, fx: n.fx, fy: n.fy });
      }
      for (const n of nodes) {
        const existing = existingMap.get(n.id);
        if (existing) {
          if (existing.x !== undefined) n.x = existing.x;
          if (existing.y !== undefined) n.y = existing.y;
          if (existing.vx !== undefined) n.vx = existing.vx;
          if (existing.vy !== undefined) n.vy = existing.vy;
          if (existing.fx !== undefined && existing.fx !== null) n.fx = existing.fx;
          if (existing.fy !== undefined && existing.fy !== null) n.fy = existing.fy;
        }
      }

      const isInitial = currentNodes.length === 0;
      currentNodes = nodes;
      currentEdges = edges;
      simulation.nodes(nodes);
      applyEdges();
      simulation.alpha(isInitial ? 1 : 0.4).restart();
    },
    updateParameters: (newParams: Partial<PhysicsParameters>) => {
      params = { ...params, ...newParams };
      manyBody.strength(-params.repulsion);
      forceX.strength(params.centerGravity);
      forceY.strength(params.centerGravity);
      link.distance(params.linkDistance);
      collide.radius((d) => params.collisionRadius + (d.betweenness || 0) * 12 + (d.hopDistance === 0 ? 10 : 0));
      simulation.alpha(0.35).restart();
    },
    getParameters: () => ({ ...params }),
    resize: (newWidth: number, newHeight: number) => {
      currentWidth = newWidth;
      currentHeight = newHeight;
      center.x(newWidth / 2).y(newHeight / 2);
      forceX.x(newWidth / 2);
      forceY.y(newHeight / 2);
      simulation.alpha(0.15).restart();
    },
    onDragStart: (node: NoteNode) => {
      simulation.alphaTarget(0.3).restart();
      node.fx = node.x ?? null;
      node.fy = node.y ?? null;
    },
    onDrag: (node: NoteNode, x: number, y: number) => {
      node.fx = x;
      node.fy = y;
    },
    onDragEnd: (node: NoteNode, x: number, y: number, pin: boolean = true) => {
      simulation.alphaTarget(0);
      if (pin) {
        node.fx = x;
        node.fy = y;
        simulation.alpha(0.2).restart();
      } else {
        node.fx = null;
        node.fy = null;
      }
    },
    unpinNode: (node: NoteNode) => {
      node.fx = null;
      node.fy = null;
      simulation.alpha(0.35).restart();
    },
    unpinAll: () => {
      for (const n of currentNodes) {
        n.fx = null;
        n.fy = null;
      }
      simulation.alpha(0.4).restart();
    },
    getPinnedCount: () => {
      return currentNodes.filter((n) => n.fx !== null && n.fx !== undefined).length;
    },
    resetDisposition: () => {
      params = { ...DEFAULT_PHYSICS_PARAMS };
      manyBody.strength(-params.repulsion);
      forceX.strength(params.centerGravity);
      forceY.strength(params.centerGravity);
      link.distance(params.linkDistance);
      collide.radius((d) => params.collisionRadius + (d.betweenness || 0) * 12 + (d.hopDistance === 0 ? 10 : 0));

      const centerX = currentWidth / 2;
      const centerY = currentHeight / 2;
      for (let i = 0; i < currentNodes.length; i++) {
        const n = currentNodes[i];
        n.fx = null;
        n.fy = null;
        const angle = i * 2.399963229728653; // Golden ratio angle
        const r = 10 * Math.sqrt(0.5 + i);
        n.x = centerX + r * Math.cos(angle);
        n.y = centerY + r * Math.sin(angle);
        n.vx = 0;
        n.vy = 0;
      }

      simulation.nodes(currentNodes);
      applyEdges();
      simulation.alpha(1).restart();
    },
    restart: (alpha = 0.5) => {
      simulation.alpha(alpha).restart();
    },
    stop: () => {
      simulation.stop();
    },
  };
}
