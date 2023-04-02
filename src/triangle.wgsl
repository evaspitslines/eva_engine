struct UniformParams {
    matrix: mat4x4<f32>
}

@group(0) @binding(0)
var<uniform> uniform_params: UniformParams;

struct VSMain {
    @location(0) position: vec3<f32>
}

struct VSOutput {
    @builtin(position) position: vec4<f32>
}

@vertex
fn vs_main(input: VSMain) -> VSOutput {
    var output: VSOutput;
    output.position = uniform_params.matrix * vec4<f32>(input.position, 1.0);

    return output;
}

@fragment
fn fs_main() -> @location(0) vec4<f32> {
    return vec4<f32>(1.0, 0.0, 0.0, 1.0);
}