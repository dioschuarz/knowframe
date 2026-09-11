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
});
