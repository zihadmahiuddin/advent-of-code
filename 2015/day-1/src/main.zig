const std = @import("std");

const input = @embedFile("input.txt");

pub fn main() !void {
    var first_index_to_enter_basement: ?usize = null;
    var floor: isize = 0;

    for (input, 0..) |value, i| {
        switch (value) {
            '(' => floor += 1,
            ')' => floor -= 1,
            else => unreachable,
        }

        if (first_index_to_enter_basement == null and floor == -1) {
            first_index_to_enter_basement = i + 1;
        }
    }

    try std.io.getStdOut()
        .writer()
        .print(
        \\The instructions take Santa to floor {}
        \\First character that causes him to enter the basement: {?}
        \\
    , .{ floor, first_index_to_enter_basement });
}
