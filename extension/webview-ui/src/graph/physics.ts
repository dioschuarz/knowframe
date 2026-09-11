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
  restart: (alpha?: number) => void;
  stop: () => void;
}

export function createPhysicsSimulation(
  width: number,
  height: number,
  onTick: () => void,
  initialParams?: Partial<PhysicsParameters>,
): PhysicsSimulation {
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
    const validEdges = currentEdges.filter(
      (e) =>
        !e.isBroken &&
        nodeMap.has(typeof e.source === 'object' ? (e.source as NoteNode).id : (e.source as string)) &&
        nodeMap.has(typeof e.target === 'object' ? (e.target as NoteNode).id : (e.target as string)),
    );
    link.links(validEdges);
  }

  return {
    simulation,
    updateData: (nodes: NoteNode[], edges: NoteEdge[]) => {
      // Preserve any user-pinned coordinates across reloads
      const existingPins = new Map<string, { fx: number; fy: number }>();
      for (const n of currentNodes) {
        if (n.fx !== null && n.fx !== undefined && n.fy !== null && n.fy !== undefined) {
          existingPins.set(n.id, { fx: n.fx, fy: n.fy });
        }
      }
      for (const n of nodes) {
        const pin = existingPins.get(n.id);
        if (pin) {
          n.fx = pin.fx;
          n.fy = pin.fy;
        }
      }

      currentNodes = nodes;
      currentEdges = edges;
      simulation.nodes(nodes);
      applyEdges();
      simulation.alpha(1).restart();
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
    restart: (alpha = 0.5) => {
      simulation.alpha(alpha).restart();
    },
    stop: () => {
      simulation.stop();
    },
  };
}
