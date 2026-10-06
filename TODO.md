# PumpkinPlus TODO

This file tracks stub modules and features that are blocked by missing Pumpkin plugin APIs.

## Stub Modules — Entity

### 1. Bat Mechanic (`src/modules/mechanics/entity/bat.rs`)

- [ ] Blocked: `EntityDeathEvent` needs entity type, killer reference, drops list
- [ ] Blocked: Game rule API (`SPAWN_PHANTOMS` check)
- **Behavior:** Drop phantom membranes on bat death (0-1 + Looting bonus)

### 2. Husk Mechanic (`src/modules/mechanics/entity/husk.rs`)

- [ ] Blocked: `EntityDeathEvent` needs entity type, killer reference, drops list
- **Behavior:** Drop sand on husk death (0-2, bonus for camel riders, +Looting)

### 3. Spawn Egg Mechanic (`src/modules/mechanics/entity/spawn_egg.rs`)

- [ ] Blocked: `EntityDeathEvent` needs entity type, drops list
- [ ] Blocked: Entity type to spawn egg item conversion
- **Behavior:** 0.1% chance to drop matching spawn egg on entity death

### 4. Tameable Mechanic (`src/modules/mechanics/entity/tameable.rs`)

- [ ] Partial: Event fires and lead check works
- [ ] Blocked: No `get_leashed_entities()` API
- [ ] Blocked: No `set_owner()` API
- [ ] Blocked: No `set_leash_holder()` API
- [ ] Blocked: No generic tameable check (only wolf/cat have `is_tamed`)
- **Behavior:** Transfer pet ownership between players using lead

### 5. Silence Mechanic (`src/modules/mechanics/entity/silence.rs`)

- [ ] Partial: Event fires and amethyst shard check works
- [ ] Blocked: No monster marker interface (need entity type whitelist)
- [ ] Blocked: No particle color options (`dust` exists but no RGB)
- **Behavior:** Toggle mob silent state with amethyst shard

## Stub Modules — Player

### 6. Anvil Mechanic (`src/modules/mechanics/player/anvil.rs`)

- [ ] Blocked: `InventoryOpenEvent` and `PrepareAnvilEvent` APIs
- [ ] Blocked: `AnvilView` / `AnvilInventory` access and modification
- [ ] Blocked: `ItemStack.set_item_on_cursor()` API
- [ ] Blocked: Enchantment data components (read/write stored enchantments)
- [ ] Blocked: `Player.give_exp_levels()` for XP cost deduction
- [ ] Blocked: Player abilities packet for creative mode bypass
- **Behavior:** Disenchant items to books, bypass cost limit, MiniMessage rename

### 7. Condense Mechanic (`src/modules/mechanics/player/condense.rs`)

- [x] Partial: Commands work (`/condense`, `/uncondense`)
- [x] Partial: 19-item condensable map implemented
- [x] Partial: Inventory iteration and conversion logic works
- [ ] Blocked: `World.drop_item()` for overflow items (currently discarded)
- [ ] Blocked: Item max stack size lookup (hardcoded to 64)
- **Behavior:** Convert 9 items → 1 block and reverse

### 8. Head Mechanic (`src/modules/mechanics/player/head.rs`)

- [ ] Blocked: No `World.drop_item()` API
- [ ] Blocked: No item profile/NBT API for player head texture
- **Behavior:** 1% chance to drop player head on death

### 9. Locator Mechanic (`src/modules/mechanics/player/locator.rs`)

- [ ] Blocked: No waypoint/locator color APIs
- [ ] Blocked: No hex color command argument parsing
- **Behavior:** `/locator <color|hex|reset>` to customize locator bar

### 10. XP Mechanic (`src/modules/mechanics/player/xp.rs`)

- [ ] Blocked: No XP point manipulation APIs
- [ ] Blocked: No `World.drop_item()` API
- **Behavior:** Right-click enchanting table with bottle to convert 11 XP

## Stub Modules — Server

### 9. Rules Mechanic (`src/modules/mechanics/server/rules.rs`)

- [ ] Blocked: No programmatic book creation (only from player hand)
- [ ] Blocked: No `Player.open_book()` without item requirement
- **Behavior:** `/rules` command opens written book with server rules

## Stub Modules — Enchantments

### 10. Tether Enchantment (`src/modules/enchantments/utility/tether.rs`)

- [ ] Blocked: Custom enchantment registration API
- [ ] Blocked: `BlockDropItemEvent` and `EntityDeathEvent` APIs
- [ ] Blocked: Player inventory addition APIs
- **Behavior:** Teleport block drops and mob XP to player inventory

## Stub Modules — Items

### 11. Incendium Key (`src/modules/items/incendium_key.rs`)

- [x] Partial: `ItemStack.set_custom_name()` works via data component API
- [x] Partial: Gradient implemented using `TextComponent.gradient()`
- [ ] Blocked: `ItemModel` component requires resource pack assets
- **Behavior:** Custom trial key with gradient red-orange name, custom model

### 12. Nullscape Key (`src/modules/items/nullscape_key.rs`)

- [x] Partial: `ItemStack.set_custom_name()` works via data component API
- [x] Partial: Gradient implemented using `TextComponent.gradient()`
- [ ] Blocked: `ItemModel` component requires resource pack assets
- **Behavior:** Custom trial key with gradient purple name, custom model

## Non-Portable Systems

### 13. Custom Painting Variants (`IllyriaPlus` `src/paintings/`)

- [ ] **Not portable** — requires registry APIs that don't exist in Pumpkin
- Blocked: No `RegistryEvents.PAINTING_VARIANT` equivalent
- Blocked: No custom `Art`/painting variant registration
- Blocked: No resource pack hosting for custom textures
- Blocked: No asset ID registration system
- **Behavior:** Register custom painting variants (e.g., Orthodox icons, Yapetto art)
- **Note:** Only vanilla painting recipes can be stubbed (already done in `src/modules/recipes/vanilla/painting.rs`)

### 14. Custom Banner Patterns (`IllyriaPlus` `src/banners/`)

- [ ] **Not portable** — requires registry APIs that don't exist in Pumpkin
- Blocked: No `RegistryEvents.BANNER_PATTERN` equivalent
- Blocked: No `PatternType` registration API
- Blocked: No resource pack hosting for custom banner textures
- Blocked: No translation key registration system
- **Behavior:** Register custom banner patterns (Moxvallix's "Many More Banners" pack with 35+ patterns)

## API Requests for Pumpkin

The following APIs need to be added to the Pumpkin plugin API to complete these modules:

| Priority | API                                                          | Modules Blocked              |
| -------- | ------------------------------------------------------------ | ---------------------------- |
| High     | Anvil events and view APIs (`InventoryOpen`, `PrepareAnvil`) | Anvil                        |
| High     | `EntityDeathEvent` with entity type, killer, drops list      | Bat, Husk, Spawn Egg, Tether |
| High     | `World.drop_item(pos, item)`                                 | Head, XP                     |
| High     | `Entity.set_silent()` / leash APIs                           | Silence, Tameable            |
| Medium   | Player XP point manipulation                                 | XP                           |
| Medium   | Book creation and opening APIs                               | Rules                        |
| Medium   | Waypoint/locator color APIs                                  | Locator                      |
| Medium   | `ItemStack` item model component (needs resource pack)       | Incendium Key, Nullscape Key |
| Low      | Particle color/dust options                                  | Silence                      |
| Low      | Generic tameable entity trait                                | Tameable                     |

## Implementation Notes

- All stubs have full config wiring and are registered in the module system
- Event handlers that can partially work (lead check, shard check) are implemented
- Debug logging is in place to verify event firing
- Each stub documents the intended behavior from IllyriaPlus as reference
