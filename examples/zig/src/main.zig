const std = @import("std");

const sokoban = @cImport(@cInclude("sokoban.h"));

pub fn main() !void {
    const allocator = std.heap.page_allocator;

    // Mock solana account. This will have u64 alignment
    var account_bytes = try allocator.alloc(u64, 4128 / @sizeOf(u64));
    defer allocator.free(account_bytes);
    // Zero-initialize the allocated memory
    for (account_bytes) |*byte| {
        byte.* = 0;
    }
    std.debug.print("allocated and zero-initialized\n", .{});

    var tree = @as(*sokoban.RedBlackTreeu64u64128, @ptrCast(&account_bytes[0]));
    sokoban.initialize(tree);
    std.debug.print("init\n", .{});

    try unwrapInsert(sokoban.c_insert(tree, 5, 3));
    std.debug.print("inserted 5 and 3\n", .{});

    try unwrapInsert(sokoban.c_insert(tree, 6, 4));
    std.debug.print("insert 6 and 4\n", .{});

    var key: u64 = 6;
    var value: u64 = 0;
    try unwrapGet(sokoban.c_get(tree, &key, &value));
    std.debug.print("get {d} -> {d}\n", .{ key, value });
}

fn unwrapGet(result: u32) !void {
    if (result != 0) {
        std.debug.print("FAILED: c_get\n", .{});
        return error.Failed;
    }
}

fn unwrapInsert(result: u32) !void {
    if (result == 0 or result == std.math.maxInt(u32)) {
        std.debug.print("FAILED: c_insert\n", .{});
        return error.Failed;
    }
}
