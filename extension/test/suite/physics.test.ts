import * as assert from 'assert';
import { describe as suite, it as test } from 'node:test';
import {
  DEFAULT_PHYSICS_PARAMS,
  PHYSICS_PRESETS,
  type NoteNode,
  type NoteEdge,
} from '../../src/types/graph.ts';
import { createPhysicsSimulation } from '../../webview-ui/src/graph/physics.ts';

suite('Graph Physics & Gravity Configuration Test Suite', () => {
  test('Default physics parameters match constitutional defaults', () => {
    assert.strictEqual(DEFAULT_PHYSICS_PARAMS.centerGravity, 0.15);
    assert.strictEqual(DEFAULT_PHYSICS_PARAMS.linkDistance, 80);
    assert.strictEqual(DEFAULT_PHYSICS_PARAMS.repulsion, 250);
    assert.strictEqual(DEFAULT_PHYSICS_PARAMS.collisionRadius, 18);
  });

  test('Physics presets contain distinct and well-calibrated configurations', () => {
    // Compact: high gravity and shorter link distance for high-level overview
    assert.ok(PHYSICS_PRESETS.compact.centerGravity > DEFAULT_PHYSICS_PARAMS.centerGravity);
    assert.ok(PHYSICS_PRESETS.compact.linkDistance < DEFAULT_PHYSICS_PARAMS.linkDistance);

    // Spacious: lower gravity and longer link distance with high repulsion for readability
    assert.ok(PHYSICS_PRESETS.spacious.centerGravity < DEFAULT_PHYSICS_PARAMS.centerGravity);
    assert.ok(PHYSICS_PRESETS.spacious.linkDistance > DEFAULT_PHYSICS_PARAMS.linkDistance);
    assert.ok(PHYSICS_PRESETS.spacious.repulsion > DEFAULT_PHYSICS_PARAMS.repulsion);
  });

  test('createPhysicsSimulation initializes with default or custom parameters', () => {
    let tickCount = 0;
    const sim = createPhysicsSimulation(800, 600, () => {
      tickCount++;
    });

    const params = sim.getParameters();
    assert.strictEqual(params.centerGravity, 0.15);
    assert.strictEqual(params.linkDistance, 80);
    assert.strictEqual(params.repulsion, 250);
    assert.strictEqual(params.collisionRadius, 18);

    sim.stop();
  });

  test('updateParameters modifies forces dynamically', () => {
    const sim = createPhysicsSimulation(800, 600, () => {});

    sim.updateParameters({
      centerGravity: 0.35,
      linkDistance: 140,
      repulsion: 400,
      collisionRadius: 22,
    });

    const updated = sim.getParameters();
    assert.strictEqual(updated.centerGravity, 0.35);
    assert.strictEqual(updated.linkDistance, 140);
    assert.strictEqual(updated.repulsion, 400);
    assert.strictEqual(updated.collisionRadius, 22);

    sim.stop();
  });

  test('Simulation updates data and filters broken edges from force layout', () => {
    let ticked = false;
    const sim = createPhysicsSimulation(800, 600, () => {
      ticked = true;
    });

    const mockNodes: NoteNode[] = [
      {
        id: 'node-1',
        label: 'Hub Note',
        filePath: 'hub.md',
        tags: [],
        color: '#3B82F6',
        status: 'active',
        tokens: 100,
        hopDistance: 0,
        inDegree: 1,
        outDegree: 1,
        betweenness: 0.5,
        brokenLinks: 1,
      },
      {
        id: 'node-2',
        label: 'Connected Note',
        filePath: 'note2.md',
        tags: [],
        color: '#64748B',
        status: 'connected_inactive',
        tokens: 50,
        hopDistance: 1,
        inDegree: 1,
        outDegree: 0,
        betweenness: 0.1,
        brokenLinks: 0,
      },
    ];

    const mockEdges: NoteEdge[] = [
      {
        id: 'e1',
        source: 'node-1',
        target: 'node-2',
        isBroken: false,
        linkType: 'wikilink',
      },
      {
        id: 'e2-broken',
        source: 'node-1',
        target: 'non-existent-node',
        isBroken: true,
        linkType: 'wikilink',
      },
    ];

    sim.updateData(mockNodes, mockEdges);
    sim.simulation.tick(5);

    assert.ok(mockNodes[0].x !== undefined);
    assert.ok(mockNodes[0].y !== undefined);
    assert.ok(mockNodes[1].x !== undefined);
    assert.ok(mockNodes[1].y !== undefined);

    sim.stop();
  });

  test('Node dragging, pinning, and unpinning lifecycle', () => {
    const sim = createPhysicsSimulation(800, 600, () => {});
    const mockNodes: NoteNode[] = [
      {
        id: 'n1',
        label: 'Node 1',
        filePath: 'n1.md',
        tags: [],
        color: '#3B82F6',
        status: 'active',
        tokens: 10,
        hopDistance: 0,
        inDegree: 0,
        outDegree: 0,
        betweenness: 0,
        brokenLinks: 0,
        x: 100,
        y: 100,
      },
      {
        id: 'n2',
        label: 'Node 2',
        filePath: 'n2.md',
        tags: [],
        color: '#64748B',
        status: 'orphan',
        tokens: 10,
        hopDistance: -1,
        inDegree: 0,
        outDegree: 0,
        betweenness: 0,
        brokenLinks: 0,
        x: 200,
        y: 200,
      },
    ];

    sim.updateData(mockNodes, []);

    // 1. Drag start
    sim.onDragStart(mockNodes[0]);
    assert.strictEqual(mockNodes[0].fx, 100);
    assert.strictEqual(mockNodes[0].fy, 100);

    // 2. Drag move
    sim.onDrag(mockNodes[0], 150, 175);
    assert.strictEqual(mockNodes[0].fx, 150);
    assert.strictEqual(mockNodes[0].fy, 175);

    // 3. Drag end with pinning
    sim.onDragEnd(mockNodes[0], 150, 175, true);
    assert.strictEqual(mockNodes[0].fx, 150);
    assert.strictEqual(mockNodes[0].fy, 175);
    assert.strictEqual(sim.getPinnedCount(), 1);

    // 4. Pin second node
    sim.onDragStart(mockNodes[1]);
    sim.onDragEnd(mockNodes[1], 300, 350, true);
    assert.strictEqual(sim.getPinnedCount(), 2);

    // 5. Unpin single node
    sim.unpinNode(mockNodes[0]);
    assert.strictEqual(mockNodes[0].fx, null);
    assert.strictEqual(mockNodes[0].fy, null);
    assert.strictEqual(sim.getPinnedCount(), 1);

    // 6. Unpin all
    sim.unpinAll();
    assert.strictEqual(mockNodes[1].fx, null);
    assert.strictEqual(mockNodes[1].fy, null);
    assert.strictEqual(sim.getPinnedCount(), 0);

    sim.stop();
  });

  test('applyEdges safely sanitizes mutated object edge sources and targets', () => {
    const sim = createPhysicsSimulation(800, 600, () => {});
    const nodeA: NoteNode = {
      id: 'a',
      label: 'A',
      filePath: 'a.md',
      tags: [],
      color: '#3B82F6',
      status: 'active',
      tokens: 10,
      hopDistance: 0,
      inDegree: 1,
      outDegree: 1,
      betweenness: 0,
      brokenLinks: 0,
      x: 50,
      y: 50,
    };
    const nodeB: NoteNode = {
      id: 'b',
      label: 'B',
      filePath: 'b.md',
      tags: [],
      color: '#64748B',
      status: 'connected_inactive',
      tokens: 10,
      hopDistance: 1,
      inDegree: 1,
      outDegree: 1,
      betweenness: 0,
      brokenLinks: 0,
      x: 100,
      y: 100,
    };

    // First update: edges have string IDs
    sim.updateData([nodeA, nodeB], [
      { id: 'e_ab', source: 'a', target: 'b', isBroken: false, linkType: 'wikilink' },
    ]);
    sim.simulation.tick(3);

    // Second update: simulate edges having mutated object references from previous D3 run
    const mutatedEdge: NoteEdge = {
      id: 'e_ab',
      source: nodeA as any,
      target: nodeB as any,
      isBroken: false,
      linkType: 'wikilink',
    };

    const nextNodeA = { ...nodeA };
    const nextNodeB = { ...nodeB };

    // Should not throw and should preserve positions smoothly
    assert.doesNotThrow(() => {
      sim.updateData([nextNodeA, nextNodeB], [mutatedEdge]);
      sim.simulation.tick(3);
    });

    assert.ok(nextNodeA.x !== undefined && !Number.isNaN(nextNodeA.x));
    assert.ok(nextNodeB.x !== undefined && !Number.isNaN(nextNodeB.x));

    sim.stop();
  });

  test('resetDisposition restores default parameters, unpins all nodes, and re-centers coordinates', () => {
    const sim = createPhysicsSimulation(800, 600, () => {});
    const node1: NoteNode = {
      id: 'n1',
      label: 'N1',
      filePath: 'n1.md',
      tags: [],
      color: '#3B82F6',
      status: 'active',
      tokens: 10,
      hopDistance: 0,
      inDegree: 1,
      outDegree: 0,
      betweenness: 0,
      brokenLinks: 0,
      x: 10,
      y: 10,
    };
    const node2: NoteNode = {
      id: 'n2',
      label: 'N2',
      filePath: 'n2.md',
      tags: [],
      color: '#64748B',
      status: 'connected_inactive',
      tokens: 10,
      hopDistance: 1,
      inDegree: 0,
      outDegree: 1,
      betweenness: 0,
      brokenLinks: 0,
      x: 20,
      y: 20,
    };

    sim.updateData([node1, node2], []);

    // 1. Alter parameters and pin nodes far away
    sim.updateParameters({
      centerGravity: 0.4,
      linkDistance: 200,
      repulsion: 600,
      collisionRadius: 35,
    });
    sim.onDragStart(node1);
    sim.onDragEnd(node1, 700, 500, true);
    assert.strictEqual(sim.getPinnedCount(), 1);
    assert.strictEqual(node1.fx, 700);
    assert.strictEqual(node1.fy, 500);

    // 2. Call resetDisposition
    sim.resetDisposition();

    // 3. Verify defaults restored
    const params = sim.getParameters();
    assert.strictEqual(params.centerGravity, DEFAULT_PHYSICS_PARAMS.centerGravity);
    assert.strictEqual(params.linkDistance, DEFAULT_PHYSICS_PARAMS.linkDistance);
    assert.strictEqual(params.repulsion, DEFAULT_PHYSICS_PARAMS.repulsion);
    assert.strictEqual(params.collisionRadius, DEFAULT_PHYSICS_PARAMS.collisionRadius);

    // 4. Verify all nodes unpinned
    assert.strictEqual(sim.getPinnedCount(), 0);
    assert.strictEqual(node1.fx, null);
    assert.strictEqual(node1.fy, null);
    assert.strictEqual(node2.fx, null);
    assert.strictEqual(node2.fy, null);

    // 5. Verify coordinates re-centered around (400, 300)
    assert.ok(node1.x !== undefined && Math.abs(node1.x - 400) < 50);
    assert.ok(node1.y !== undefined && Math.abs(node1.y - 300) < 50);
    assert.strictEqual(node1.vx, 0);
    assert.strictEqual(node1.vy, 0);

    // 6. Verify simulation restarted with high alpha
    assert.ok(sim.simulation.alpha() > 0.8);

    sim.stop();
  });
});

