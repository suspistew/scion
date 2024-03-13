/*
    This file contains a custom exporter to use in tiled and that is compatible with the scion custom integration.
    This handle :
        - Compatible structure with ScionTiledImporter
        - Mandatory field check
        - Mandatory properties on maps

    To use the ScionTiledImporter you must use this exporter, althouht if you need missing features from Tiled, consider
    making your own importer.
 */

const OBJECT_TYPES = ["Collider", "Item", "Door", "Trigger"];
const MAP_SHAPES = {
    0: 'Rectangle',
    1: 'Polygon',
    2: 'Polyline',
    3: 'Ellipse',
    4: 'Text',
    5: 'Point'
}

/*
================= Custom format exporters =================
*/

tiled.registerMapFormat("scion", {
    name: "Scion engine format", extension: "scion",

    write: (map, fileName) => {
        let jsonFormattedMap;
        try {
            let formattedMap = buildScionMap(map);
            jsonFormattedMap = JSON.stringify(formattedMap, (key, value) => {
                if (value !== null) return value
            });
            let file = new TextFile(fileName, TextFile.WriteOnly);
            file.write(jsonFormattedMap);
            file.commit();
        } catch (e) {
            return e;
        }
        return null;
    },
});

tiled.registerTilesetFormat("scion", {
    name: "Scion engine format", extension: "scion",

    write: (tileset, fileName) => {
        let jsonFormattedMap;
        try {
            let formattedMap = buildScionTileset(tileset);
            jsonFormattedMap = JSON.stringify(formattedMap, (key, value) => {
                if (value !== null) return value
            });
            let file = new TextFile(fileName, TextFile.WriteOnly);
            file.write(jsonFormattedMap);
            file.commit();
        } catch (e) {
            return e;
        }
        return null;
    },
});


/*
================= Map exporter utils =================
*/
function buildScionMap(map) {
    let tilesets = buildMapTilesets(map);
    return {
        type: 'Map',
        width: map.width,
        height: map.height,
        tile_width: map.tileWidth,
        tile_height: map.tileHeight,
        properties: map.properties(),
        layers: buildLayers(map, tilesets),
        objects: buildObjects(map),
        tilesets: tilesets
    };
}

function buildLayers(map, tilesets) {
    let layers = [];
    let flatLayers = flattenLayers(map.layers);
    for (let i = 0; i < flatLayers.length; ++i) {
        const currentLayer = flatLayers[i];
        if (currentLayer.isTileLayer) {
            layers.push(buildSingleLayer(currentLayer, tilesets));
        }
    }
    return layers;
}

function flattenLayers(layers) {
    let flatLayers = [];
    for (let i = 0; i < layers.length; ++i) {
        if (layers[i].isGroupLayer) {
            Array.prototype.push.apply(flatLayers, flattenLayers(layers[i].layers));
        } else {
            flatLayers.push(layers[i]);
        }
    }
    return flatLayers;
}

function buildSingleLayer(layer, tilesets) {
    let layerTiles = [];
    for (let y = 0; y < layer.height; ++y) {
        const row = [];
        for (let x = 0; x < layer.width; ++x) {
            let tile = layer.tileAt(x, y);
            if (tile) {
                let tileset = findMatchingTileset(tile, tilesets);
                let offset = computeOffsetId(tilesets, tileset);
                row.push(tile.id + offset);
            } else {
                row.push(-1);
            }
        }
        layerTiles.push(row);
    }


    return {
        name: layer.name,
        tiles: Base64.encode(JSON.stringify(layerTiles)),
        properties: layer.properties(),
    };
}

function findMatchingTileset(tile, tilesets) {
    return tilesets.find((tileset) => tileset.name === tile.tileset.name);
}

function computeOffsetId(tilesets, tileset) {
    let offset = 0;
    for (let i = 0; i < tileset.index; i++) {
        offset += tilesets[i].totalTiles;
    }
    return offset;
}

function buildMapTilesets(map) {
    let tilesets = [];
    for (let i = 0; i < map.tilesets.length; i++) {
        let current_tileset = map.tilesets[i];
        tilesets[i] = {
            index: i,
            totalTiles: current_tileset.tileCount,
            name: current_tileset.name
        };
    }
    return tilesets;
}

function buildObjects(map) {
    let objects = [];
    let flatLayers = flattenLayers(map.layers);
    for (let i = 0; i < map.layerCount; ++i) {
        const currentLayer = flatLayers[i];
        if (currentLayer.isObjectLayer) {
            currentLayer.objects
                .forEach(o => {
                    if (!hasClass(o)) {
                        throw new Error("Object at x:" + o.x + ", y:" + o.y + " is missing its class property");
                    }
                    if (!hasName(o)) {
                        throw new Error("Object at x:" + o.x + ", y:" + o.y + " is missing its name property");
                    }
                    let obj = computeObject(o);
                    if (obj) {
                        objects.push(obj);
                    }
                });
        }
    }
    return objects;
}

function hasClass(object) {
    return object
        && object.className
        && object.className.length !== 0;
}

function hasName(object) {
    return object
        && object.name
        && object.name !== 0;
}

function mapClass(object) {
    let objectType = object.className;
    const firstLetterCap = objectType.charAt(0).toUpperCase();
    const remainingLetters = objectType.slice(1).toLowerCase();
    objectType = firstLetterCap + remainingLetters;
    if (OBJECT_TYPES.indexOf(objectType) !== -1) {
        return objectType;
    }
    return "Custom(" + objectType + ")";
}

function computeObject(o) {
    const shapeType = MAP_SHAPES[o.shape];
    let rectangle = null, polygon = null;

    switch (shapeType) {
        case 'Rectangle':
            rectangle = {
                width: o.width,
                height: o.height
            }
            break;
        case 'Polygon':
            polygon = {
                coordinates: o.polygon && o.polygon.length > 0 ? o.polygon : null
            }
            break;
    }

    let props = o.resolvedProperties();
    let i = 0;
    for (let _ in props) {
        i += 1;
    }

    return {
        name: o.name,
        class: mapClass(o),
        shapeType: shapeType,
        position: {
            x: o.x,
            y: o.y
        },
        properties: i > 0 ? props : null,
        polygon: polygon,
        rectangle: rectangle,
    };
}

/*
================= Tileset exporter utils =================
*/

function buildScionTileset(tileset) {
    return {
        type: 'Tilemap',
        totalTiles: tileset.tileCount,
        name: tileset.name,
        properties: tileset.properties(),
        tiles: buildTiles(tileset)
    }
}

function buildTiles(tileset) {
    let tiles = {};
    for (let currentTile in tileset.tiles) {
        let props = tileset.tiles[currentTile].properties();
        let i = 0;
        for (let _ in props) {
            i += 1;
        }

        let colliders = tileset.tiles[currentTile].objectGroup;
        let objects = [];
        if (colliders) {
            colliders.objects
                .forEach(o => {
                    let obj = computeObject(o);
                    if (obj) {
                        obj["name"] = null;
                        obj["class"] = "Collider";
                        objects.push(obj);
                    }
                });
        }

        if (i > 0 || tileset.tiles[currentTile].objectGroup || tileset.tiles[currentTile].objectGroup || tileset.tiles[currentTile].frames.length > 0) {
            tiles[currentTile] = {
                properties: i > 0 ? props : null,
                animation: tileset.tiles[currentTile].frames.length > 0 ? tileset.tiles[currentTile].frames : null,
                objects: objects
            }
        }
    }
    return tiles;
}