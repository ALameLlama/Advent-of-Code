local function part1()
	local total = 0
	local grid = {}
	for line in io.lines("input.txt") do
		table.insert(grid, line)
	end

	local len = string.len(grid[1])
	local empty_line = string.rep(".", len)

	for x = 1, #grid do
		local line = grid[x]

		for y = 1, len do
			local char = line:sub(y, y)

			if char ~= "@" then
				goto continue
			end

			local start_pos = math.max(1, y - 1)
			local end_pos = math.min(len, y + 1)

			local count = 0
			local top_line = grid[x - 1] or empty_line
			local bottom_line = grid[x + 1] or empty_line

			for i = start_pos, end_pos do
				if top_line:sub(i, i) == "@" then count = count + 1 end
				if line:sub(i, i) == "@" then count = count + 1 end
				if bottom_line:sub(i, i) == "@" then count = count + 1 end

				if count > 4 then
					goto continue
				end
			end
			total = total + 1

			::continue::
		end
	end

	print("Part 1: " .. total)
end

local function part2()
	local total = 0
	local grid = {}
	for line in io.lines("input.txt") do
		table.insert(grid, line)
	end

	local len = string.len(grid[1])
	local empty_line = string.rep(".", len)

	local diff = true
	while diff do
		local diff_total = total

		for x = 1, #grid do
			local line = grid[x]

			for y = 1, len do
				local char = line:sub(y, y)
				if char ~= "@" then
					goto continue
				end

				local start_pos = math.max(1, y - 1)
				local end_pos = math.min(len, y + 1)

				local count = 0
				local top_line = grid[x - 1] or empty_line
				local bottom_line = grid[x + 1] or empty_line

				for i = start_pos, end_pos do
					if top_line:sub(i, i) == "@" then count = count + 1 end
					if line:sub(i, i) == "@" then count = count + 1 end
					if bottom_line:sub(i, i) == "@" then count = count + 1 end

					if count > 4 then
						goto continue
					end
				end

				total = total + 1

				grid[x] = line:sub(1, y - 1) .. 'x' .. line:sub(y + 1)
				line = grid[x]

				::continue::
			end
		end
		if diff_total == total then
			diff = false
		end
	end

	print("Part 2: " .. total)
end

part1()
part2()
