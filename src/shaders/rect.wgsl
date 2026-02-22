// Глобальні дані для всього вікна
struct Globals {
    projection: mat4x4<f32>,
};

// Дані конкретного віджета (Rectangle)
struct RectUniform {
    pos: vec2<f32>,
    size: vec2<f32>,
    color: vec4<f32>,
};

// Реєструємо групи вхідних даних
@group(0) @binding(0) var<uniform> globals: Globals;
@group(1) @binding(0) var<uniform> rect: RectUniform;

struct VertexOutput {
    // Вбудована змінна для позиції в просторі GPU (-1.0 до 1.0)
    @builtin(position) clip_pos: vec4<f32>,
    // Передаємо колір у фрагментний шейдер
    @location(0) color: vec4<f32>,
};

@vertex
fn vs_main(@location(0) pos: vec2<f32>) -> VertexOutput {
    var out: VertexOutput;
    
    // 1. Беремо базову вершину квада (0.0 або 1.0)
    // 2. Множимо на розмір віджета в пікселях (наприклад, 200.0)
    // 3. Додаємо позицію віджета в пікселях (наприклад, x: 50, y: 50)
    let pixel_pos = (pos * rect.size) + rect.pos;
    
    // 4. Множимо на матрицю проекції, щоб перетворити пікселі в координати GPU
    // Результат буде в межах [-1.0, 1.0] для X та [1.0, -1.0] для Y
    out.clip_pos = globals.projection * vec4<f32>(pixel_pos, 0.0, 1.0);
    
    // Передаємо колір далі
    out.color = rect.color;
    
    return out;
}

@fragment
fn fs_main(in: VertexOutput) -> @location(0) vec4<f32> {
    // Просто зафарбовуємо піксель кольором віджета
    return in.color;
}
