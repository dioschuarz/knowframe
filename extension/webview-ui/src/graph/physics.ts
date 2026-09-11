import * as d3 from 'd3-force';
import { NoteNode, NoteEdge } from '../../../src/types/graph';

export interface PhysicsSimulation {
  simulation: d3.Simulation<NoteNode, NoteEdge>;
  updateData: (nodes: NoteNode[], edges: NoteEdge[]) => void;
  restart: () => void;
  stop: () => void;
}

export function createPhysicsSimulation(
  width: number,
  height: number,
  onTick: () => void,
): PhysicsSimulation {
  const simulation = d3
    .forceSimulation<NoteNode, NoteEdge>()
    .force('charge', d3.forceManyBody().strength(-200))
    .force('center', d3.forceCenter(width / 2, height / 2))
    .force(
      'collision',
      d3.forceCollide<NoteNode>().radius((d) => 12 + (d.betweenness || 0) * 15 + (d.hopDistance === 0 ? 12 : 0)),
    )
    .force(
      'link',
      d3
        .forceLink<NoteNode, NoteEdge>()
        .id((d) => d.id)
        .distance(80),
    )
    .on('tick', onTick);

  return {
    simulation,
    updateData: (nodes: NoteNode[], edges: NoteEdge[]) => {
      simulation.nodes(nodes);
      const linkForce = simulation.force('link') as d3.ForceLink<NoteNode, NoteEdge>;
      if (linkForce) {
        // Only include non-broken links between existing nodes in force layout
        const nodeMap = new Set(nodes.map((n) => n.id));
        const validEdges = edges.filter(
          (e) =>
            !e.isBroken &&
            nodeMap.has(typeof e.source === 'object' ? e.source.id : e.source) &&
            nodeMap.has(typeof e.target === 'object' ? e.target.id : e.target),
        );
        linkForce.links(validEdges);
      }
      simulation.alpha(1).restart();
    },
    restart: () => {
      simulation.alpha(1).restart();
    },
    stop: () => {
      simulation.stop();
    },
  };
}
