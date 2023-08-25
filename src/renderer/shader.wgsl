struct VertexInput {
  @location(0) position: vec3<f32>,
  @location(1) texture_coordinates: vec2<f32>,
}
struct VertexOutput {
  @builtin(position) clip_position: vec4<f32>,
  @location(0) texture_coordinates: vec2<f32>,
};

@vertex
fn vs_main(
    model: VertexInput,
) -> VertexOutput {
  var out: VertexOutput;
  out.texture_coordinates = model.texture_coordinates;
  out.clip_position = vec4<f32>(model.position, 1.0);
  return out;
}

@group(0) @binding(0)
var texture_diffuse: texture_2d<f32>;
@group(0) @binding(1)
var sample_diffuse: sampler; 


@fragment
fn fs_main(in: VertexOutput) -> @location(0) vec4<f32> {
  return textureSample(texture_diffuse, sample_diffuse, in.texture_coordinates);
}
