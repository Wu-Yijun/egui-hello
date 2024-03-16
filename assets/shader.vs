const vec2 verts[4] =
    vec2[4](vec2(0.0, 1.0), vec2(-1.0, 0.0), vec2(1.0, 0.0), vec2(0.0, -1.0));
// const vec4 colors[4] =
//     vec4[4](vec4(1.0, 0.5, 0.0, 0.5), vec4(0.0, 1.0, 0.5, 0.5),
//             vec4(0.5, 0.0, 1.0, 0.5), vec4(1.0, 0.5, 0.0, 0.5));
const int ids[4] = int[4](1, 0, 3, 2);
out vec4 v_color;
uniform float u_angle;
uniform int u_order;
uniform mat4 u_colors;

void main() {
  if (u_order % 2 == 0) {
    v_color = u_colors[gl_VertexID];
    gl_Position = vec4(verts[gl_VertexID], 0.0, 1.0);
    gl_Position.x *= cos(u_angle);
  } else {
    v_color = u_colors[ids[gl_VertexID]];
    gl_Position = vec4(verts[ids[gl_VertexID]], 0.0, 1.0);
    gl_Position.x *= cos(u_angle);
  }
}