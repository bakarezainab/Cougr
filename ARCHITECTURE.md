# Cougr Core Architecture

This document provides a detailed overview of the complete Cougr Core architecture, including all 127 modules across 16 directories.

## Complete Module Structure

### Root Modules (src/)
- **lib.rs** - Main library entry point with module exports and prelude
- **entity.rs** - Core entity types (Entity, EntityId)
- **component.rs** - Component types and trait definitions
- **world.rs** - Main World container
- **system.rs** - System trait and core functionality
- **storage.rs** - Base storage types
- **query.rs** - Query functionality
- **resource.rs** - Global resource management
- **event.rs** - Event types and core functionality
- **archetype.rs** - Archetype-based entity organization
- **bundle.rs** - Component bundle system
- **batching.rs** - Batch processing utilities
- **change_detection.rs** - Component change tracking
- **components.rs** - Built-in component types
- **systems.rs** - Built-in system implementations
- **entity_disabling.rs** - Entity enable/disable functionality
- **hierarchy.rs** - Entity hierarchy and parent-child relationships
- **intern.rs** - String interning utilities
- **label.rs** - System and schedule labeling
- **lifecycle.rs** - Entity lifecycle management
- **name.rs** - Named entity functionality
- **never.rs** - Never type utilities
- **spawn.rs** - Entity spawning utilities
- **traversal.rs** - Hierarchy traversal utilities

### Entity Module (src/entity/)
Advanced entity management and collections:
- **clone_entities.rs** - Entity cloning functionality
- **entity_set.rs** - Set collection for entities
- **hash.rs** - Entity hashing utilities
- **hash_map.rs** - HashMap for entities
- **hash_set.rs** - HashSet for entities
- **index_map.rs** - IndexMap for entities
- **index_set.rs** - IndexSet for entities
- **map_entities.rs** - Entity mapping utilities
- **unique_array.rs** - Unique array storage
- **unique_slice.rs** - Unique slice utilities
- **unique_vec.rs** - Unique vector storage

### Error Module (src/error/)
Comprehensive error handling:
- **mod.rs** - Error module exports
- **bevy_error.rs** - Core error types
- **command_handling.rs** - Command execution errors
- **handler.rs** - Error handler implementations

### Event Module (src/event/)
Advanced event system:
- **base.rs** - Base event types
- **collections.rs** - Event collections
- **event_cursor.rs** - Event cursor for iteration
- **iterators.rs** - Event iterators
- **mut_iterators.rs** - Mutable event iterators
- **mutator.rs** - Event mutation utilities
- **reader.rs** - Event reader implementation
- **registry.rs** - Event type registry
- **update.rs** - Event update system
- **writer.rs** - Event writer implementation

### Observer Module (src/observer/)
Observer pattern implementation:
- **mod.rs** - Observer module exports
- **centralized_storage.rs** - Centralized observer storage
- **distributed_storage.rs** - Distributed observer storage
- **entity_cloning.rs** - Observer for entity cloning
- **runner.rs** - Observer execution
- **system_param.rs** - Observer system parameters
- **trigger_targets.rs** - Observer trigger targeting

### Query Module (src/query/)
Advanced querying system:
- **access.rs** - Component access tracking
- **builder.rs** - Query builder pattern
- **error.rs** - Query error types
- **fetch.rs** - Data fetching utilities
- **filter.rs** - Query filters
- **iter.rs** - Query iterators
- **par_iter.rs** - Parallel query iteration
- **state.rs** - Query state management
- **world_query.rs** - World query trait

### Reflect Module (src/reflect/)
Reflection and introspection:
- **mod.rs** - Reflect module exports
- **bundle.rs** - Bundle reflection
- **component.rs** - Component reflection
- **entity_commands.rs** - Reflected entity commands
- **from_world.rs** - FromWorld trait reflection
- **map_entities.rs** - Entity mapping reflection
- **resource.rs** - Resource reflection

### Relationship Module (src/relationship/)
Entity relationship system:
- **mod.rs** - Relationship module exports
- **related_methods.rs** - Methods for related entities
- **relationship_query.rs** - Querying relationships
- **relationship_source_collection.rs** - Relationship source management

### Schedule Module (src/schedule/)
System scheduling and execution:
- **mod.rs** - Schedule module exports
- **auto_insert_apply_deferred.rs** - Automatic deferred command application
- **condition.rs** - System run conditions
- **config.rs** - Schedule configuration
- **pass.rs** - Schedule pass system
- **schedule.rs** - Main schedule implementation
- **set.rs** - System sets
- **stepping.rs** - Schedule stepping/debugging

#### Schedule/Executor (src/schedule/executor/)
Different execution strategies:
- **mod.rs** - Executor module exports
- **simple.rs** - Simple sequential executor
- **single_threaded.rs** - Single-threaded executor
- **multi_threaded.rs** - Multi-threaded parallel executor

#### Schedule/Graph (src/schedule/graph/)
Dependency graph for system scheduling:
- **mod.rs** - Graph module exports
- **graph_map.rs** - Graph data structure
- **node.rs** - Graph node implementation
- **tarjan_scc.rs** - Strongly connected components (cycle detection)

### Storage Module (src/storage/)
Component storage implementations:
- **blob_array.rs** - Blob array storage
- **blob_vec.rs** - Blob vector storage
- **resource.rs** - Resource storage
- **sparse_set.rs** - Sparse set storage

#### Storage/Table (src/storage/table/)
Table-based storage:
- **mod.rs** - Table module exports
- **column.rs** - Table column implementation

### System Module (src/system/)
Advanced system functionality:
- **adapter_system.rs** - System adapters
- **builder.rs** - System builder pattern
- **combinator.rs** - System combinators
- **exclusive_function_system.rs** - Exclusive access systems
- **exclusive_system_param.rs** - Exclusive system parameters
- **function_system.rs** - Function-based systems
- **input.rs** - System input handling
- **observer_system.rs** - Observer-based systems
- **query.rs** - System query parameters
- **schedule_system.rs** - Scheduled system execution
- **system.rs** - Core system trait
- **system_name.rs** - System naming
- **system_param.rs** - System parameter trait
- **system_registry.rs** - System type registry

#### System/Commands (src/system/commands/)
Command pattern for deferred operations:
- **mod.rs** - Commands module exports
- **command.rs** - Core command trait
- **entity_command.rs** - Entity-specific commands
- **parallel_scope.rs** - Parallel command execution

### World Module (src/world/)
Advanced world functionality:
- **command_queue.rs** - Command queue implementation
- **deferred_world.rs** - Deferred world access
- **entity_fetch.rs** - Entity fetching utilities
- **entity_ref.rs** - Entity references (EntityRef, EntityMut)
- **error.rs** - World error types
- **filtered_resource.rs** - Filtered resource access
- **identifier.rs** - World identifiers
- **reflect.rs** - World reflection
- **spawn_batch.rs** - Batch entity spawning
- **unsafe_world_cell.rs** - Unsafe world cell for parallel access

## Key Design Patterns

### 1. Archetype-Based Storage
Entities with the same component types are grouped into archetypes for efficient storage and iteration.

### 2. Command Pattern
Deferred operations through the command system allow safe mutation during system execution.

### 3. Observer Pattern
Observers react to entity/component changes, enabling reactive programming.

### 4. Reflection System
Complete introspection capabilities for components, resources, and systems.

### 5. Schedule Graph
Dependency-based system scheduling with automatic parallelization.

### 6. Query System
Powerful filtering and iteration over entities with specific component combinations.

### 7. Relationship System
First-class support for entity relationships and hierarchies.

### 8. Change Detection
Automatic tracking of component modifications for efficient updates.

## Soroban Adaptations

All modules have been adapted for Soroban compatibility:
- **no_std** environment
- **WASM** target support
- **wee_alloc** for memory management
- **soroban-sdk** type integration
- Simplified threading (single-threaded executor focus)
- Optimized for blockchain constraints

## Module Dependencies

The architecture follows a layered approach:
1. **Foundation**: entity, component, storage
2. **Core**: world, query, system
3. **Advanced**: schedule, observer, reflect
4. **Utilities**: error, event, relationship
5. **Optimizations**: batching, change_detection, archetype

## Total Statistics

- **127 Rust files**
- **16 directories**
- **8 major subsystems**
- Fully integrated with Soroban SDK 23.0.2
- Compiles successfully with only warnings
