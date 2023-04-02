struct VSMain {
    @location(0) position: vec2<f32>
}

struct VSOutput {
    @builtin(position) position: vec4<f32>
}

@vertex
fn vs_main(input: VSMain) -> VSOutput {
    var output: VSOutput;
    output.position = vec4<f32>(input.position, 0.0, 1.0);

    return output;
}

@fragment
fn fs_main() -> @location(0) vec4<f32> {
    return vec4<f32>(1.0, 0.0, 0.0, 1.0);
}