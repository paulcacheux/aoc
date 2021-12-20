A = [
    [0, 1, 2],
    [1, 2, 0],
    [2, 0, 1]
]

B = [[1, 1, 1],
     [1, -1, -1],
     [-1, 1, -1],
     [-1, -1, 1]]

C = [1, -1]

template = """
pub fn {fn_name}(point: Vec3) -> Vec3 {{
    vector![{components}]
}}
"""

res_buffer = ""
res_buffer += "use crate::aoc2021::day19::Vec3;\n"
res_buffer += "use nalgebra::vector;\n"

counter_id = 0
fn_names = []
for order in A:
    for coeff in B:
        for side in C:
            components = []
            for o, c in zip(order[::side], coeff[::side]):
                sign = c * side
                if sign == 1:
                    components.append(f"point[{o}]")
                else:
                    components.append(f"-point[{o}]")

            components = ", ".join(components)
            fn_name = f"rot{counter_id}"
            fn_names.append(fn_name)
            res_buffer += template.format(fn_name=fn_name,
                                          components=components)
            counter_id += 1

res_buffer += "\npub const ROTATIONS: [fn(Vec3) -> Vec3; 24] = ["
for name in fn_names:
    res_buffer += f"    {name},\n"
res_buffer += "];"

print(res_buffer)
