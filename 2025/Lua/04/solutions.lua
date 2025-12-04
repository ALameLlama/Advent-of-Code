local function part1()
	local total = 0
	local grid = {}
	for line in io.lines("input.txt") do
		table.insert(grid, line)
	end

	for x = 1, #grid do
		local line = grid[x]
		local len = string.len(line)
		local empty_line = string.rep(".", len)
		for y = 1, len do
			local count = 0

			local char = line:sub(y, y)
			if char ~= "@" then
				goto continue
			end

			local start_pos = y
			if start_pos ~= 1 then
				start_pos = start_pos - 1
			end

			local end_pos = y
			if end_pos ~= len then
				end_pos = end_pos + 1
			end

			local top_line = grid[x - 1] or empty_line
			local top_chars = top_line:sub(start_pos, end_pos)
			local _, top_count = top_chars:gsub("@", "")
			count = count + top_count

			local current_chars = line:sub(start_pos, end_pos)
			local _, current_count = current_chars:gsub("@", "")
			count = count + current_count

			if count > 4 then
				goto continue
			end

			local bottom_line = grid[x + 1] or empty_line
			local bottom_chars = bottom_line:sub(start_pos, end_pos)
			local _, bottom_count = bottom_chars:gsub("@", "")
			count = count + bottom_count

			if count > 4 then
				goto continue
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


	local diff = true
	while diff do
		local diff_total = total
		for x = 1, #grid do
			local line = grid[x]
			local len = string.len(line)
			local empty_line = string.rep(".", len)
			for y = 1, len do
				local count = 0

				local char = line:sub(y, y)
				if char ~= "@" then
					goto continue
				end

				local start_pos = y
				if start_pos ~= 1 then
					start_pos = start_pos - 1
				end

				local end_pos = y
				if end_pos ~= len then
					end_pos = end_pos + 1
				end

				local top_line = grid[x - 1] or empty_line
				local top_chars = top_line:sub(start_pos, end_pos)
				local _, top_count = top_chars:gsub("@", "")
				count = count + top_count

				local current_chars = line:sub(start_pos, end_pos)
				local _, current_count = current_chars:gsub("@", "")
				count = count + current_count

				if count > 4 then
					goto continue
				end

				local bottom_line = grid[x + 1] or empty_line
				local bottom_chars = bottom_line:sub(start_pos, end_pos)
				local _, bottom_count = bottom_chars:gsub("@", "")
				count = count + bottom_count

				if count > 4 then
					goto continue
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
