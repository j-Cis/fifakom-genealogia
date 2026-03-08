# RAPORT KODU: doc/code-slint.md

## Plik-RustBuild: `build.rs`

```rust
fn main() {
    //let config = slint_build::CompilerConfiguration::default();
    slint_build::compile("src/ui/index.slint").expect("Błąd kompilacji interfejsu (index.slint)");
}

```

## Plik-SlintApp_01: `src/ui/app/app-geneteka-zakres.slint`

```slint
import { Kolory } from "../theme/kolory.slint";
import {
    MapaKanwa,
    MapaKokpit,
    MapaCelownik,
    MapaLogika,
    MapaWarstwyZiemi,
    MapaWarstwyDanych,
} from "../element/mapa.slint";
import { OknoSzkielet, OknoGuzik } from "../element/okno.slint";
import { LineEdit } from "std-widgets.slint";

export component AppGenetekaZakres inherits OknoSzkielet {
    preferred-width: 1600px;
    preferred-height: 1000px;
    window-title: "GENETEKA - CO JEST?";
    callback search(string);
    in property <image> map_frame;
    callback camera_changed(float, float, float, float, float, float);
    in-out property <[string]> search_results: [];
    in property <float> geo_min_lon;
    in property <float> geo_max_lon;
    in property <float> geo_min_lat;
    in property <float> geo_max_lat;
    VerticalLayout {
        padding: 20px;
        spacing: 15px;
        HorizontalLayout {
            spacing: 10px;
            height: 40px;
            input := LineEdit {
                placeholder-text: "Szukaj parafii...";
                accepted => {
                    root.search(self.text);
                }
            }

            OknoGuzik {
                text: "SZUKAJ";
                width: 100px;
                primary: true;
                clicked => {
                    root.search(input.text);
                }
            }
        }

        kanwa := MapaKanwa {
            map_frame: root.map_frame;
            camera_changed(w, h, ox, oy, z, rot) => {
                root.camera_changed(w, h, ox, oy, z, rot);
            }
            MapaCelownik {
                width: 100%;
                height: 100%;
                n_coord: MapaLogika.current_lat(kanwa.width, kanwa.height, kanwa.map_offset_x, kanwa.map_offset_y, kanwa.map_zoom, kanwa.map_rotation, root.geo_min_lat, root.geo_max_lat);
                e_coord: MapaLogika.current_lon(kanwa.width, kanwa.height, kanwa.map_offset_x, kanwa.map_offset_y, kanwa.map_zoom, kanwa.map_rotation, root.geo_min_lon, root.geo_max_lon);
                zoom_str: MapaLogika.current_zoom(kanwa.map_zoom);
                rot_str: MapaLogika.current_rot(kanwa.map_rotation);
                anchor: "ne";
            }

            MapaKokpit {
                anchor: "sw";
                margin: 10px;
                x: (self.anchor == "nw" || self.anchor == "sw") ? self.margin : parent.width - self.width - self.margin;
                y: (self.anchor == "nw" || self.anchor == "ne") ? self.margin : parent.height - self.height - self.margin;
                zoom_in => {
                    kanwa.zoom_around_center(kanwa.map_zoom * 1.15);
                }
                zoom_out => {
                    kanwa.zoom_around_center(kanwa.map_zoom / 1.15);
                }
                rotate_ccw => {
                    kanwa.rotate_view(-5deg);
                }
                rotate_cw => {
                    kanwa.rotate_view(5deg);
                }
                pan(dx, dy) => {
                    kanwa.map_offset_x += dx;
                    kanwa.map_offset_y += dy;
                    kanwa.trigger_render();
                }
                reset => {
                    kanwa.map_zoom = 1.0;
                    kanwa.map_offset_x = 0px;
                    kanwa.map_offset_y = 0px;
                    kanwa.map_rotation = 0deg;
                    kanwa.trigger_render();
                }
            }

            MapaWarstwyZiemi {
                margin: 10px;
                x: parent.width - self.width - self.margin;
                y: parent.height - self.height - self.margin;
                warstwy_zmienione => {
                    kanwa.trigger_render();
                }
            }

            MapaWarstwyDanych {
                margin: 10px;
                // Odsunięte o dodatkowe 40px od prawej krawędzi (35px guzik + 5px odstępu)
                x: parent.width - self.width - self.margin - 40px;
                y: parent.height - self.height - self.margin;
                warstwy_zmienione => {
                    kanwa.trigger_render();
                }
            }
        }
    }
}

```

## Plik-SlintApp_02: `src/ui/app/app-kartezjan-produkt.slint`

```slint
import { Kolory } from "../theme/kolory.slint";
import { OknoSzkielet, OknoGuzik } from "../element/okno.slint";
import { LineEdit, ScrollView } from "std-widgets.slint";

export component AppKartezjanProdukt inherits OknoSzkielet {
    preferred-width: 600px;
    preferred-height: 300px;
    window-title: "CPSGen - FORMATOWANIE";
    in-out property <[string]> results: [];
    in-out property <string> joined_text: "";
    in-out property <bool> show_as_list: true;
    callback send(string);
    callback copy_to_clipboard(string);
    VerticalLayout {
        padding: 20px;
        spacing: 15px;
        alignment: start;
        input := LineEdit {
            placeholder-text: "Wzór...";
            height: 35px;
            accepted => {
                root.send(self.text);
            }
        }

        HorizontalLayout {
            spacing: 10px;
            height: 35px;
            OknoGuzik {
                text: "LISTA";
                enabled: !root.show_as_list;
                clicked => {
                    root.show_as_list = true;
                }
            }

            OknoGuzik {
                text: "TEKST";
                enabled: root.show_as_list;
                clicked => {
                    root.show_as_list = false;
                }
            }
        }

        OknoGuzik {
            text: "GENERUJ";
            height: 40px;
            primary: true;
            clicked => {
                root.send(input.text);
            }
        }

        Rectangle {
            background: Kolory.background-light;
            border-radius: 4px;
            vertical-stretch: 1;
            clip: true;
            ScrollView {
                viewport-width: self.width;
                if (root.show_as_list): VerticalLayout {
                    padding: 10px;
                    spacing: 5px;
                    for res in root.results: Text {
                        text: res;
                        color: Kolory.accent-success;
                    }
                }
                if (!root.show_as_list): VerticalLayout {
                    padding: 10px;
                    Text {
                        text: root.joined_text;
                        color: Kolory.accent-success;
                        wrap: word-wrap;
                    }
                }
            }
        }

        OknoGuzik {
            text: "KOPIUJ WYNIKI";
            clicked => {
                root.copy_to_clipboard(root.show_as_list ? "list" : "text");
            }
        }
    }
}

```

## Plik-SlintApp_03: `src/ui/app/app.slint`

```slint
export { AppGenetekaZakres } from "app-geneteka-zakres.slint";
export { AppKartezjanProdukt } from "app-kartezjan-produkt.slint";
```

## Plik-SlintElement_01: `src/ui/element/mapa-celownik.slint`

```slint
import { Kolory } from "../theme/kolory.slint";

export component MapaCelownik inherits Rectangle {
    in property <string> n_coord: "0.0000°";
    in property <string> e_coord: "0.0000°";
    in property <string> zoom_str: "1.0x";
    in property <string> rot_str: "0°";
    in property <string> anchor: "ne";
    in property <length> margin: 10px;
    Rectangle {
        width: 20px;
        height: 20px;
        x: (parent.width - self.width) / 2;
        y: (parent.height - self.height) / 2;
        Path {
            commands: "M 10 0 L 10 20 M 0 10 L 20 10";
            stroke: Kolory.accent-success.with-alpha(0.3);
            stroke-width: 1px;
        }
    }

    Rectangle {
        x: (root.anchor == "nw" || root.anchor == "sw") ? root.margin : parent.width - self.width - root.margin;
        y: (root.anchor == "nw" || root.anchor == "ne") ? root.margin : parent.height - self.height - root.margin;
        background: Kolory.background-light.with-alpha(0.7);
        border-radius: 4px;
        width: coords_layout.preferred-width + 10px;
        height: coords_layout.preferred-height + 10px;
        coords_layout := VerticalLayout {
            padding: 5px;
            Text {
                text: root.n_coord + " | " + root.e_coord + " | R: " + root.rot_str + " | Z: " + root.zoom_str;
                color: Kolory.accent-success;
                font-family: "Fira Code";
                font-size: 14px;
            }
        }
    }
}

```

## Plik-SlintElement_02: `src/ui/element/mapa-kanwa-logika.slint`

```slint
// Struktura przechowująca "stan" kamery po przeliczeniu
export struct MapaKameraStan {
    offset_x: length,
    offset_y: length,
    zoom: float,
    rotation: angle,
}

export global MapaKanwaLogika {
    // CZYSTA FUNKCJA: Obliczanie nowego stanu po obrocie
    public pure function oblicz_obrot(w: length, h: length, ox: length, oy: length, z: float, r: angle, d_angle: angle) -> MapaKameraStan {
        let view_center_x = w / 2;
        let view_center_y = h / 2;
        let target_x = view_center_x - ox;
        let target_y = view_center_y - oy;
        let pivot_x = (w * z) / 2;
        let pivot_y = (h * z) / 2;
        let dx = target_x - pivot_x;
        let dy = target_y - pivot_y;
        let unrot_cos = Math.cos(-r);
        let unrot_sin = Math.sin(-r);
        let orig_dx = dx * unrot_cos - dy * unrot_sin;
        let orig_dy = dx * unrot_sin + dy * unrot_cos;
        let new_r = r + d_angle;
        let new_cos = Math.cos(new_r);
        let new_sin = Math.sin(new_r);
        let new_dx = orig_dx * new_cos - orig_dy * new_sin;
        let new_dy = orig_dx * new_sin + orig_dy * new_cos;
        let new_target_x = pivot_x + new_dx;
        let new_target_y = pivot_y + new_dy;
        return {
            offset_x: view_center_x - new_target_x,
            offset_y: view_center_y - new_target_y,
            zoom: z,
            rotation: new_r
        };
    }

    // CZYSTA FUNKCJA: Obliczanie nowego stanu po powiększeniu
    public pure function oblicz_zoom(w: length, h: length, ox: length, oy: length, old_zoom: float, new_zoom: float, rot: angle) -> MapaKameraStan {
        if (new_zoom <= 0) {
            return { offset_x: ox, offset_y: oy, zoom: old_zoom, rotation: rot };
        }
        let visual_center_x = w / 2;
        let visual_center_y = h / 2;
        let s = new_zoom / old_zoom;
        let new_center_inner_x = (visual_center_x - ox) * s;
        let new_center_inner_y = (visual_center_y - oy) * s;
        return {
            offset_x: visual_center_x - new_center_inner_x,
            offset_y: visual_center_y - new_center_inner_y,
            zoom: new_zoom,
            rotation: rot
        };
    }
}

```

## Plik-SlintElement_03: `src/ui/element/mapa-kanwa-punkt.slint`

```slint
export struct MapaKanwaPunkt {
    x: float,
    y: float,
    nazwa: string,
}
```

## Plik-SlintElement_04: `src/ui/element/mapa-kanwa.slint`

```slint
import { Kolory } from "../theme/kolory.slint";
import { MapaKanwaLogika, MapaKameraStan } from "mapa-kanwa-logika.slint";

export component MapaKanwa inherits Rectangle {
    in property <image> map_frame;
    in-out property <float> map_zoom: 1.0;
    in-out property <length> map_offset_x: 0px;
    in-out property <length> map_offset_y: 0px;
    in-out property <angle> map_rotation: 0deg;
    callback camera_changed(float, float, float, float, float, float);
    horizontal-stretch: 1;
    background: Kolory.background-light;
    border-radius: 4px;
    border-width: 1px;
    border-color: Kolory.accent-success;
    clip: true;
    public function trigger_render() {
        root.camera_changed(
            root.width / 1px, root.height / 1px,
            root.map_offset_x / 1px, root.map_offset_y / 1px,
            root.map_zoom, root.map_rotation / 1deg);
    }
    // Wywołanie zewnętrznej logiki
    public function rotate_view(d_angle: angle) {
        let stan = MapaKanwaLogika.oblicz_obrot(
            root.width, root.height, 
            root.map_offset_x, root.map_offset_y, 
            root.map_zoom, root.map_rotation, d_angle);
        root.map_offset_x = stan.offset_x;
        root.map_offset_y = stan.offset_y;
        root.map_rotation = stan.rotation;
        root.trigger_render();
    }

    // Wywołanie zewnętrznej logiki
    public function zoom_around_center(new_zoom: float) {
        let stan = MapaKanwaLogika.oblicz_zoom(
            root.width, root.height, 
            root.map_offset_x, root.map_offset_y, 
            root.map_zoom, new_zoom, root.map_rotation);
        root.map_offset_x = stan.offset_x;
        root.map_offset_y = stan.offset_y;
        root.map_zoom = stan.zoom;
        root.trigger_render();
    }
    TouchArea {
        width: 100%;
        height: 100%;
        property <length> start_x;
        property <length> start_y;
        pointer-event(event) => {
            if (event.kind == PointerEventKind.down) {
                self.start_x = root.map_offset_x;
                self.start_y = root.map_offset_y;
            }
        }
        moved => {
            root.map_offset_x = self.start_x + (self.mouse-x - self.pressed-x);
            root.map_offset_y = self.start_y + (self.mouse-y - self.pressed-y);
            root.trigger_render();
        }
        scroll-event(event) => {
            if (event.delta-y > 0px) {
                root.zoom_around_center(root.map_zoom - 0.2);
            }
            if (event.delta-y < 0px) {
                root.zoom_around_center(root.map_zoom + 0.2);
            }
            return accept;
        }
    }

    Image {
        width: 100%;
        height: 100%;
        source: root.map_frame;
        image-fit: fill;
    }

    changed width => {
        root.trigger_render();
    }
    changed height => {
        root.trigger_render();
    }
    @children
}

```

## Plik-SlintElement_05: `src/ui/element/mapa-kokpit-guzik.slint`

```slint
import { Kolory } from "../theme/kolory.slint";

export component MapaKokpitGuzik inherits Rectangle {
    in property <string> icon_commands;
    in property <bool> is_filled: false;
    in property <bool> is_active: false;
    callback clicked();
    width: 35px;
    height: 35px;
    background: root.is_active ? Kolory.accent-success.with-alpha(0.3) : (ta.pressed ? Kolory.accent-success.darker(0.2) : (ta.has-hover ? Kolory.background-light.brighter(0.1) : Kolory.background-light));
    border-radius: 4px;
    border-width: 1px;
    border-color: Kolory.accent-success;
    Path {
        width: 24px;
        height: 24px;
        x: (parent.width - self.width) / 2;
        y: (parent.height - self.height) / 2;
        commands: root.icon_commands;
        stroke: root.is_filled ? transparent : Kolory.text-primary;
        fill: root.is_filled ? Kolory.text-primary : transparent;
        stroke-width: root.is_filled ? 0px : 2px;
    }

    ta := TouchArea {
        clicked => {
            root.clicked();
        }
    }
}

```

## Plik-SlintElement_06: `src/ui/element/mapa-kokpit-siatka.slint`

```slint
import { MapaKokpitGuzik } from "mapa-kokpit-guzik.slint";

export component MapaKokpitSiatka inherits GridLayout {
    padding: 0px;
    spacing: 5px;
    in property <string> anchor: "sw";
    in property <length> margin: 10px;
    in-out property <bool> keyboard_active: false;
    callback zoom_in();
    callback zoom_out();
    callback pan(length, length);
    callback reset();
    callback rotate_ccw();
    callback rotate_cw();
    callback toggle_keyboard();
    // RZĄD 1 (Q, W, E)
    Row {
        MapaKokpitGuzik {
            icon_commands: "M 10 5 L 12 5 L 12 10 L 17 10 L 17 12 L 12 12 L 12 17 L 10 17 L 10 12 L 5 12 L 5 10 L 10 10 Z";
            is_filled: true;
            clicked => {
                root.zoom_in();
            }
        }

        MapaKokpitGuzik {
            icon_commands: "M 11 6 L 17 14 L 5 14 Z";
            is_filled: true;
            clicked => {
                root.pan(0px, 100px);
            }
        }

        MapaKokpitGuzik {
            icon_commands: "M 5 12 L 17 12 L 17 10 L 5 10 Z";
            is_filled: true;
            clicked => {
                root.zoom_out();
            }
        }
    }
    // RZĄD 2 (A, S, D)
    Row {
        MapaKokpitGuzik {
            icon_commands: "M 6 12 L 14 6 L 14 18 Z";
            is_filled: true;
            clicked => {
                root.pan(100px, 0px);
            }
        }

        MapaKokpitGuzik {
            icon_commands: "M 11 18 L 17 10 L 5 10 Z";
            is_filled: true;
            clicked => {
                root.pan(0px, -100px);
            }
        }

        MapaKokpitGuzik {
            icon_commands: "M 16 12 L 8 6 L 8 18 Z";
            is_filled: true;
            clicked => {
                root.pan(-100px, 0px);
            }
        }
    }
    // RZĄD 3 (Z, X, C)
    Row {
        MapaKokpitGuzik {
            icon_commands: "M11,1 L5,6 L11,11 L11,8 A4,4 0 1,1 7,12 L3,12 A8,8 0 1,0 11,4 Z";
            is_filled: true;
            clicked => {
                root.rotate_ccw();
            }
        }

        VerticalLayout {
            spacing: 5px;
            alignment: center;

            // GÓRNY MAŁY GUZIK - Reset 
            MapaKokpitGuzik {
                height: 15px;
                icon_commands: "M 6 2 L 17 2 L 17 8 L 6 8 Z";
                is_filled: true;
                clicked => {
                    root.reset();
                }
            }

            // DOLNY MAŁY GUZIK - Toggle Klawiatury
            MapaKokpitGuzik {
                height: 15px;
                icon_commands: "";
                is_filled: true;
                is_active: root.keyboard_active;
                clicked => {
                    // Wywołujemy callback zamiast samemu łapać focus!
                    root.toggle_keyboard();
                }
            }
        }

        MapaKokpitGuzik {
            icon_commands: "M11,1 L17,6 L11,11 L11,8 A4,4 0 1,0 15,12 L19,12 A8,8 0 1,1 11,4 Z";
            is_filled: true;
            clicked => {
                root.rotate_cw();
            }
        }
    }
}

```

## Plik-SlintElement_07: `src/ui/element/mapa-kokpit.slint`

```slint
import { MapaKokpitGuzik } from "mapa-kokpit-guzik.slint";
import { MapaKokpitSiatka } from "mapa-kokpit-siatka.slint";

export component MapaKokpit inherits FocusScope {
    // padding: 0px;
    in property <string> anchor: "sw";
    in property <length> margin: 10px;
    in-out property <bool> keyboard_active: false;
    // Rozmiar dopasowany do GridLayoutu w środku
    width: grid.preferred-width;
    height: grid.preferred-height;
    callback zoom_in();
    callback zoom_out();
    callback pan(length, length);
    callback reset();
    callback rotate_ccw();
    callback rotate_cw();


    // NASŁUCHIWANIE KLAWISZY (QWE / ASD / ZXC)
    key-pressed(event) => {
        
        // WYŁĄCZANIE KLAWIATURY: [Alt] + [W]
        if (root.keyboard_active && event.modifiers.alt && (event.text == "w" || event.text == "W")) {
            root.keyboard_active = false;
            return accept;
        }
        if (!root.keyboard_active) {
            return reject;
        }
        
        // Rząd górny (Q W E)
        if (event.text == "q" || event.text == "Q") {
            root.zoom_in();
            return accept;
        }
        if (event.text == "w" || event.text == "W") {
            root.pan(0px, 100px);
            return accept;
        }
        if (event.text == "e" || event.text == "E") {
            root.zoom_out();
            return accept;
        }
        
        // Rząd środkowy (A S D)
        if (event.text == "a" || event.text == "A") {
            root.pan(100px, 0px);
            return accept;
        }
        if (event.text == "s" || event.text == "S") {
            root.pan(0px, -100px);
            return accept;
        }
        if (event.text == "d" || event.text == "D") {
            root.pan(-100px, 0px);
            return accept;
        }
        
        // Rząd dolny (Z X C)
        if (event.text == "z" || event.text == "Z") {
            root.rotate_ccw();
            return accept;
        }
        // Klawisz X resetuje widok!
        if (event.text == "x" || event.text == "X") {
            // root.keyboard_active = false;
            root.reset();
            return accept;
        }
        if (event.text == "c" || event.text == "C") {
            root.rotate_cw();
            return accept;
        }
        return reject;
    }
    grid := MapaKokpitSiatka {
        anchor: root.anchor;
        margin: root.margin;
        keyboard_active: root.keyboard_active;
        // Gdy siatka każe nam przełączyć klawiaturę (kliknięto dolny guzik)
        toggle_keyboard => {
            root.keyboard_active = !root.keyboard_active;
            if (root.keyboard_active) {
                // FocusScope "skupia uwagę"
                root.focus();
            }
        }
        zoom_in => root.zoom_in();
        zoom_out => root.zoom_out();
        pan(length, length) => root.pan(length, length);
        reset => root.reset();
        rotate_ccw => root.rotate_ccw();
        rotate_cw => root.rotate_cw();
    }
}

```

## Plik-SlintElement_08: `src/ui/element/mapa-logika.slint`

```slint
export global MapaLogika {
    // Obliczanie szerokości geograficznej (N/S)
    public pure function current_lat(w: length, h: length, ox: length, oy: length, z: float, rot: angle, min_lat: float, max_lat: float) -> string {
        let view_center_x = w / 2;
        let view_center_y = h / 2;
        let target_x = view_center_x - ox;
        let target_y = view_center_y - oy;
        let pivot_x = w * z / 2;
        let pivot_y = h * z / 2;
        let dx = target_x - pivot_x;
        let dy = target_y - pivot_y;
        let cos_a = Math.cos(-rot);
        let sin_a = Math.sin(-rot);
        let orig_y = pivot_y + dx * sin_a + dy * cos_a;
        let ratio = orig_y / (h * z);
        let val = max_lat - (ratio * (max_lat - min_lat));
        let prefix = val >= 0 ? "N:" : "S:";
        let val_abs = Math.abs(val);
        return prefix + " " + Math.round(val_abs * 10000) / 10000 + "°";
    }

    // Obliczanie długości geograficznej (E/W)
    public pure function current_lon(w: length, h: length, ox: length, oy: length, z: float, rot: angle, min_lon: float, max_lon: float) -> string {
        let view_center_x = w / 2;
        let view_center_y = h / 2;
        let target_x = view_center_x - ox;
        let target_y = view_center_y - oy;
        let pivot_x = w * z / 2;
        let pivot_y = h * z / 2;
        let dx = target_x - pivot_x;
        let dy = target_y - pivot_y;
        let cos_a = Math.cos(-rot);
        let sin_a = Math.sin(-rot);
        let orig_x = pivot_x + dx * cos_a - dy * sin_a;
        let ratio = orig_x / (w * z);
        let val = min_lon + (ratio * (max_lon - min_lon));
        let prefix = val >= 0 ? "E:" : "W:";
        let val_abs = Math.abs(val);
        return prefix + " " + Math.round(val_abs * 10000) / 10000 + "°";
    }

    // Formatowanie kąta obrotu
    public pure function current_rot(rot: angle) -> string {
        let deg = Math.mod(rot / 1deg, 360);
        let final_deg = deg < 0 ? deg + 360 : deg;
        return Math.round(final_deg) + "°";
    }

    // Formatowanie poziomu powiększenia
    public pure function current_zoom(z: float) -> string {
        return (Math.round(z * 100) / 100) + "x";
    }
}

```

## Plik-SlintElement_09: `src/ui/element/mapa-warstwy-danych.slint`

```slint
import { Kolory } from "../theme/kolory.slint";
import { MapaKokpitGuzik } from "mapa-kokpit-guzik.slint";

component RadioBtn inherits Rectangle {
    in property <bool> active: false;
    callback clicked();
    width: 18px;
    height: 18px;
    border-radius: 9px;
    border-width: 2px;
    border-color: Kolory.accent-success;
    background: transparent;
    Rectangle {
        width: 10px;
        height: 10px;
        border-radius: 5px;
        background: root.active ? Kolory.accent-success : transparent;
        x: (parent.width - self.width) / 2;
        y: (parent.height - self.height) / 2;
    }

    TouchArea {
        clicked => {
            root.clicked();
        }
        mouse-cursor: pointer;
    }
}

export component MapaWarstwyDanych inherits Rectangle {
    in property <length> margin: 10px;
    in-out property <bool> is_open: false;
    
    // Dostępne opcje: "brak", "z_mapy", "z_rejestru", "suma"
    in-out property <string> opracowane_parafie: "suma"; // Domyślnie pokazujemy wszystko
    
    callback warstwy_zmienione();
    width: layout.preferred-width;
    height: layout.preferred-height;
    layout := VerticalLayout {
        alignment: end;
        spacing: 10px;
        if (root.is_open): Rectangle {
            background: Kolory.background-light.with-alpha(0.85);
            border-radius: 4px;
            border-width: 1px;
            border-color: Kolory.accent-success;
            GridLayout {
                padding: 15px;
                spacing: 15px;
                
                // Wiersz 1: Tytuł
                Row {
                    Text {
                        text: "DANE GENETEKI";
                        color: Kolory.accent-success;
                        font-weight: 700;
                        font-family: "Fira Code";
                        colspan: 5;
                    }
                }
                
                // Wiersz 2: Nagłówki kolumn
                Row {
                    Text {
                        text: "";
                    }

                    Text {
                        text: "Brak";
                        color: Kolory.accent-success;
                        font-weight: 700;
                        font-family: "Fira Code";
                        horizontal-alignment: center;
                    }

                    Text {
                        text: "Z mapy";
                        color: Kolory.accent-success;
                        font-weight: 700;
                        font-family: "Fira Code";
                        horizontal-alignment: center;
                    }

                    Text {
                        text: "Z rejestru";
                        color: Kolory.accent-success;
                        font-weight: 700;
                        font-family: "Fira Code";
                        horizontal-alignment: center;
                    }

                    Text {
                        text: "Suma";
                        color: Kolory.accent-success;
                        font-weight: 700;
                        font-family: "Fira Code";
                        horizontal-alignment: center;
                    }
                }
                
                // Wiersz 3: Opracowane parafie
                Row {
                    Text {
                        text: "Opracowane\nparafie";
                        color: Kolory.accent-success;
                        vertical-alignment: center;
                        font-family: "Fira Code";
                    }

                    RadioBtn {
                        active: root.opracowane_parafie == "brak";
                        clicked => {
                            root.opracowane_parafie = "brak";
                            root.warstwy_zmienione();
                        }
                    }

                    RadioBtn {
                        active: root.opracowane_parafie == "z_mapy";
                        clicked => {
                            root.opracowane_parafie = "z_mapy";
                            root.warstwy_zmienione();
                        }
                    }

                    RadioBtn {
                        active: root.opracowane_parafie == "z_rejestru";
                        clicked => {
                            root.opracowane_parafie = "z_rejestru";
                            root.warstwy_zmienione();
                        }
                    }

                    RadioBtn {
                        active: root.opracowane_parafie == "suma";
                        clicked => {
                            root.opracowane_parafie = "suma";
                            root.warstwy_zmienione();
                        }
                    }
                }

                // Wiersz 4: Przyszłe wyznania (wyszarzone jako "wkrótce")
                Row {
                    Text {
                        text: "Wyznania\n(Wkrótce)";
                        color: Kolory.text-secondary;
                        vertical-alignment: center;
                        font-family: "Fira Code";
                    }

                    Text {
                        text: "Rz-Kat";
                        color: Kolory.text-secondary;
                        horizontal-alignment: center;
                        vertical-alignment: center;
                        font-size: 11px;
                    }

                    Text {
                        text: "Gr-Kat";
                        color: Kolory.text-secondary;
                        horizontal-alignment: center;
                        vertical-alignment: center;
                        font-size: 11px;
                    }

                    Text {
                        text: "Prawosł";
                        color: Kolory.text-secondary;
                        horizontal-alignment: center;
                        vertical-alignment: center;
                        font-size: 11px;
                    }

                    Text {
                        text: "Inne";
                        color: Kolory.text-secondary;
                        horizontal-alignment: center;
                        vertical-alignment: center;
                        font-size: 11px;
                    }
                }
            }
        }
        HorizontalLayout {
            alignment: end;
            MapaKokpitGuzik {
                // Ikona bazy danych / listy (kropki i kreski)
                icon_commands: "M 5 6 L 7 6 L 7 8 L 5 8 Z M 10 6 L 19 6 L 19 8 L 10 8 Z M 5 11 L 7 11 L 7 13 L 5 13 Z M 10 11 L 19 11 L 19 13 L 10 13 Z M 5 16 L 7 16 L 7 18 L 5 18 Z M 10 16 L 19 16 L 19 18 L 10 18 Z";
                is_filled: true;
                is_active: root.is_open;
                clicked => {
                    root.is_open = !root.is_open;
                }
            }
        }
    }
}

```

## Plik-SlintElement_10: `src/ui/element/mapa-warstwy-ziemi.slint`

```slint
import { Kolory } from "../theme/kolory.slint";
import { MapaKokpitGuzik } from "mapa-kokpit-guzik.slint";

// Wewnętrzny komponent - okrągły przycisk "Radio"
component RadioBtn inherits Rectangle {
    in property <bool> active: false;
    callback clicked();
    width: 18px;
    height: 18px;
    border-radius: 9px;
    border-width: 2px;
    border-color: Kolory.accent-success;
    background: transparent;
    Rectangle {
        width: 10px;
        height: 10px;
        border-radius: 5px;
        background: root.active ? Kolory.accent-success : transparent;
        x: (parent.width - self.width) / 2;
        y: (parent.height - self.height) / 2;
    }

    TouchArea {
        clicked => {
            root.clicked();
        }
        mouse-cursor: pointer;
    }
}

export component MapaWarstwyZiemi inherits Rectangle {
    in property <length> margin: 10px;
    in-out property <bool> is_open: false;
    
    // Zmienne przechowujące wybraną rozdzielczość: "none", "110m", "50m", "10m"
    in-out property <string> rivers_res: "none";
    in-out property <string> lakes_res: "none";
    
    // Sygnał wysyłany do Rusta, gdy zmienimy warstwę
    callback warstwy_zmienione();
    width: layout.preferred-width;
    height: layout.preferred-height;
    layout := VerticalLayout {
        alignment: end;
        spacing: 10px;
        if (root.is_open): Rectangle {
            background: Kolory.background-light.with-alpha(0.85);
            border-radius: 4px;
            border-width: 1px;
            border-color: Kolory.accent-success;
            GridLayout {
                padding: 15px;
                spacing: 15px;
                
                // Wiersz 1: Nagłówki
                Row {
                    Text {
                        text: "WARSTWY";
                        color: Kolory.accent-success;
                        font-weight: 700;
                        font-family: "Fira Code";
                    }

                    Text {
                        text: "Brak";
                        color: Kolory.accent-success;
                        font-weight: 700;
                        font-family: "Fira Code";
                        horizontal-alignment: center;
                    }

                    Text {
                        text: "110m";
                        color: Kolory.accent-success;
                        font-weight: 700;
                        font-family: "Fira Code";
                        horizontal-alignment: center;
                    }

                    Text {
                        text: "50m";
                        color: Kolory.accent-success;
                        font-weight: 700;
                        font-family: "Fira Code";
                        horizontal-alignment: center;
                    }

                    Text {
                        text: "10m";
                        color: Kolory.accent-success;
                        font-weight: 700;
                        font-family: "Fira Code";
                        horizontal-alignment: center;
                    }
                }
                
                // Wiersz 2: Rzeki
                Row {
                    Text {
                        text: "Rzeki";
                        color: Kolory.accent-success;
                        vertical-alignment: center;
                        font-family: "Fira Code";
                    }

                    RadioBtn {
                        active: root.rivers_res == "none";
                        clicked => {
                            root.rivers_res = "none";
                            root.warstwy_zmienione();
                        }
                    }

                    RadioBtn {
                        active: root.rivers_res == "110m";
                        clicked => {
                            root.rivers_res = "110m";
                            root.warstwy_zmienione();
                        }
                    }

                    RadioBtn {
                        active: root.rivers_res == "50m";
                        clicked => {
                            root.rivers_res = "50m";
                            root.warstwy_zmienione();
                        }
                    }

                    RadioBtn {
                        active: root.rivers_res == "10m";
                        clicked => {
                            root.rivers_res = "10m";
                            root.warstwy_zmienione();
                        }
                    }
                }

                // Wiersz 3: Jeziora
                Row {
                    Text {
                        text: "Jeziora";
                        color: Kolory.accent-success;
                        vertical-alignment: center;
                        font-family: "Fira Code";
                    }

                    RadioBtn {
                        active: root.lakes_res == "none";
                        clicked => {
                            root.lakes_res = "none";
                            root.warstwy_zmienione();
                        }
                    }

                    RadioBtn {
                        active: root.lakes_res == "110m";
                        clicked => {
                            root.lakes_res = "110m";
                            root.warstwy_zmienione();
                        }
                    }

                    RadioBtn {
                        active: root.lakes_res == "50m";
                        clicked => {
                            root.lakes_res = "50m";
                            root.warstwy_zmienione();
                        }
                    }

                    RadioBtn {
                        active: root.lakes_res == "10m";
                        clicked => {
                            root.lakes_res = "10m";
                            root.warstwy_zmienione();
                        }
                    }
                }
            }
        }

        // Guzik otwierający panel w prawym dolnym rogu
        HorizontalLayout {
            alignment: end;
            MapaKokpitGuzik {
                // Ikona warstw (romby nakładające się na siebie)
                icon_commands: "M 12 4 L 4 8 L 12 12 L 20 8 Z M 4 13 L 12 17 L 20 13 M 4 17 L 12 21 L 20 17";
                is_filled: false;
                is_active: root.is_open;
                clicked => {
                    root.is_open = !root.is_open;
                }
            }
        }
    }
}

```

## Plik-SlintElement_11: `src/ui/element/mapa.slint`

```slint
export { MapaCelownik } from "mapa-celownik.slint";
export { MapaKokpit } from "mapa-kokpit.slint";
export { MapaKokpitSiatka } from "mapa-kokpit-siatka.slint";
export { MapaKokpitGuzik } from "mapa-kokpit-guzik.slint";
export { MapaKanwa } from "mapa-kanwa.slint";
export { MapaKanwaPunkt } from "mapa-kanwa-punkt.slint";
export { MapaLogika } from "mapa-logika.slint";
export { MapaKanwaLogika, MapaKameraStan } from "mapa-kanwa-logika.slint";
export { MapaWarstwyZiemi } from "mapa-warstwy-ziemi.slint";
export { MapaWarstwyDanych } from "mapa-warstwy-danych.slint";

```

## Plik-SlintElement_12: `src/ui/element/okno-granica.slint`

```slint
export component OknoGranica inherits Rectangle {
    callback resize(string);
    background: transparent;
    width: 100%;
    height: 100%;
    property <length> border-size: 8px;
    TouchArea {
        width: 100%;
        height: border-size;
        y: 0;
        mouse-cursor: MouseCursor.row-resize;
        pointer-event(e) => {
            if (e.button == PointerEventButton.left && e.kind == PointerEventKind.down) {
                root.resize("n");
            }
        }
    }

    TouchArea {
        width: 100%;
        height: border-size;
        y: parent.height - self.height;
        mouse-cursor: MouseCursor.row-resize;
        pointer-event(e) => {
            if (e.button == PointerEventButton.left && e.kind == PointerEventKind.down) {
                root.resize("s");
            }
        }
    }

    TouchArea {
        width: border-size;
        height: 100%;
        x: 0;
        mouse-cursor: MouseCursor.col-resize;
        pointer-event(e) => {
            if (e.button == PointerEventButton.left && e.kind == PointerEventKind.down) {
                root.resize("w");
            }
        }
    }

    TouchArea {
        width: border-size;
        height: 100%;
        x: parent.width - self.width;
        mouse-cursor: MouseCursor.col-resize;
        pointer-event(e) => {
            if (e.button == PointerEventButton.left && e.kind == PointerEventKind.down) {
                root.resize("e");
            }
        }
    }

    TouchArea {
        x: 0;
        y: 0;
        width: border-size * 2;
        height: border-size * 2;
        mouse-cursor: MouseCursor.nwse-resize;
        pointer-event(e) => {
            if (e.button == PointerEventButton.left && e.kind == PointerEventKind.down) {
                root.resize("nw");
            }
        }
    }

    TouchArea {
        x: parent.width - self.width;
        y: 0;
        width: border-size * 2;
        height: border-size * 2;
        mouse-cursor: MouseCursor.nesw-resize;
        pointer-event(e) => {
            if (e.button == PointerEventButton.left && e.kind == PointerEventKind.down) {
                root.resize("ne");
            }
        }
    }

    TouchArea {
        x: 0;
        y: parent.height - self.height;
        width: border-size * 2;
        height: border-size * 2;
        mouse-cursor: MouseCursor.nesw-resize;
        pointer-event(e) => {
            if (e.button == PointerEventButton.left && e.kind == PointerEventKind.down) {
                root.resize("sw");
            }
        }
    }

    TouchArea {
        x: parent.width - self.width;
        y: parent.height - self.height;
        width: border-size * 2;
        height: border-size * 2;
        mouse-cursor: MouseCursor.nwse-resize;
        pointer-event(e) => {
            if (e.button == PointerEventButton.left && e.kind == PointerEventKind.down) {
                root.resize("se");
            }
        }
    }
}

```

## Plik-SlintElement_13: `src/ui/element/okno-guzik.slint`

```slint
import { Kolory } from "../theme/kolory.slint";

export component OknoGuzik {
    in property <string> text;
    in property <bool> enabled: true;
    in property <bool> primary: false;
    in property <brush> custom-hover-bg: Kolory.hover-background;
    callback clicked;
    Rectangle {
        background: root.primary ? #00a2ff : (ta.has-hover ? root.custom-hover-bg : (root.enabled ? Kolory.background-light : Kolory.background));
        border-width: 1px;
        border-color: Kolory.border;
        border-radius: 4px;
        height: 100%;
        width: 100%;
        Text {
            text: root.text;
            color: !root.enabled ? Kolory.text-secondary : (root.primary ? white : Kolory.text-primary);
            font-weight: 700;
            vertical-alignment: center;
            horizontal-alignment: center;
            font-family: "BlexMono Nerd Font";
        }

        ta := TouchArea {
            enabled: root.enabled;
            clicked => {
                root.clicked();
            }
            mouse-cursor: self.has-hover ? pointer : default;
        }
    }
}

```

## Plik-SlintElement_14: `src/ui/element/okno-logika.slint`

```slint
export global OknoLogika {
    callback zamykanie();
    callback ukrywanie();
    callback przesuwanie(length, length);
    callback wymiarowanie(string);
}

```

## Plik-SlintElement_15: `src/ui/element/okno-pasek.slint`

```slint
import { Kolory } from "../theme/kolory.slint";
import { OknoGuzik } from "okno-guzik.slint";
import { OknoLogika } from "okno-logika.slint";


export component OknoPasek inherits Rectangle {
    in property <string> window-title: "TYTUŁ OKNA";
    height: 50px;
    background: Kolory.window-backgr-bar;
    TouchArea {
        width: 100%;
        height: 100%;
        pointer-event(event) => {
            if (event.kind == PointerEventKind.down && event.button == PointerEventButton.left) {
                OknoLogika.przesuwanie(0px, 0px);
            }
        }
    }

    HorizontalLayout {
        padding-left: 15px;
        padding-right: 15px;
        spacing: 10px;
        Text {
            text: root.window-title;
            color: Kolory.window-text-main;
            vertical-alignment: center;
            font-weight: 700;
        }

        Rectangle {
            horizontal-stretch: 1;
        }

        OknoGuzik {
            text: "\u{f00d}";
            width: 35px;
            height: 35px;
            custom-hover-bg: #ff000033;
            clicked => {
                OknoLogika.zamykanie();
            }
        }

        OknoGuzik {
            text: "—";
            width: 35px;
            height: 35px;
            custom-hover-bg: #00a2ff33;
            clicked => {
                OknoLogika.ukrywanie();
            }
        }

        OknoGuzik {
            text: Kolory.is-dark ? "☀️" : "🌙";
            width: 40px;
            height: 35px;
            clicked => {
                Kolory.is-dark = !Kolory.is-dark;
            }
        }
    }
}

```

## Plik-SlintElement_16: `src/ui/element/okno-szkielet.slint`

```slint
import { Kolory } from "../theme/kolory.slint";
import { OknoGranica } from "okno-granica.slint";
import { OknoLogika } from "okno-logika.slint";
import { OknoPasek } from "okno-pasek.slint";

export component OknoSzkielet inherits Window {
    no-frame: true;
    background: transparent;
    min-width: 200px;
    min-height: 50px;
    in property <string> window-title: "TYTUŁ OKNA";
    Rectangle {
        width: 100%;
        height: 100%;
        background: Kolory.window-backgr-box;
        border-radius: 12px;
        border-width: 2px;
        border-color: Kolory.window-border-box;
        clip: true;
        VerticalLayout {
            OknoPasek {
                window-title: root.window-title;
            }

            Rectangle {
                vertical-stretch: 1;
                @children
            }
        }

        OknoGranica {
            resize(dir) => {
                OknoLogika.wymiarowanie(dir);
            }
        }
    }
}

```

## Plik-SlintElement_17: `src/ui/element/okno.slint`

```slint

export { OknoGranica } from "okno-granica.slint";
export { OknoGuzik } from "okno-guzik.slint";
export { OknoPasek } from "okno-pasek.slint";
export { OknoSzkielet } from "okno-szkielet.slint";
export { OknoLogika } from "okno-logika.slint";
```

## Plik-SlintIndex: `src/ui/index.slint`

```slint
import { OknoLogika } from "element/okno.slint";

import { AppKartezjanProdukt, AppGenetekaZakres } from "app/app.slint";

import "font/BlexMonoNerdFont-Bold.ttf";
import "font/FiraCode-Regular.ttf"; 
import "font/RammettoOne-Regular.ttf";

export { AppKartezjanProdukt,AppGenetekaZakres, OknoLogika }


```

## Plik-SlintTheme_01: `src/ui/theme/kolory.slint`

```slint
export global Kolory {
    in-out property <bool> is-dark: true;
    out property <brush> window-backgr-box: is-dark ? #1c1c1c : #f0f0f0;
    out property <brush> window-border-box: is-dark ? #00ff00 : #026702;
    out property <brush> window-backgr-bar: is-dark ? #333333 : #dddddd;
    out property <brush> window-text-main: is-dark ? #ffffff : #1e1e1e;
    out property <brush> background: is-dark ? #1c1c1c : #f0f0f0;
    out property <brush> background-light: is-dark ? #2d2d2d : #ffffff;
    out property <brush> title-bar-background: is-dark ? #333333 : #dddddd;
    out property <brush> text-primary: is-dark ? #ffffff : #1e1e1e;
    out property <brush> text-secondary: is-dark ? #aaaaaa : #666666;
    out property <brush> border: is-dark ? #444444 : #cccccc;
    out property <brush> hover-background: is-dark ? #ffffff1a : #0000001a;
    out property <brush> accent-success: #00ff00;
    out property <brush> accent-danger: #c42b1c;
    out property <brush> accent-danger-hover: #e81123;
}

```

