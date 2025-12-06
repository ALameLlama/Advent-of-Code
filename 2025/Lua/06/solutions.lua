-- local input = "test.txt"
local input = "input.txt"

local function zip(tbl)
	local result = {}
	local num_cols = #tbl[1]

	for col = 1, num_cols do
		local new_row = {}

		for row = 1, #tbl do
			table.insert(new_row, tbl[row][col])
		end

		table.insert(result, new_row)
	end

	return result
end

local function reduce(tbl, op)
	local result = tbl[1]

	for i = 2, #tbl do
		local v = tbl[i]

		if op == "+" then
			result = result + v
		elseif op == "-" then
			result = result - v
		elseif op == "*" then
			result = result * v
			-- elseif op == "/" then
			-- 	result = result / v
		end
	end

	return result
end

local function part1()
	local total = 0
	local result = {}

	for line in io.lines(input) do
		local eq1 = {}

		for numbers in line:gmatch("%S+") do
			table.insert(eq1, numbers)
		end

		table.insert(result, eq1)
	end

	local zipped = zip(result)

	for i = 1, #zipped do
		local nums = {}
		local op = table.remove(zipped[i])

		for j = 1, #zipped[i] do
			nums[j] = tonumber(zipped[i][j])
		end

		local answer = reduce(nums, op)
		total = total + answer
	end

	print("Part 1: " .. total)
end

local function part2()
	local total = 0
	local lines = {}

	for line in io.lines(input) do
		table.insert(lines, line)
	end

	local height = #lines
	local operator_line = lines[height]

	-- loop over all the operators these are in fixed positions
	-- we can use these to split the rows above
	-- this feels kinda gross
	local op_positions = {}

	for col = 1, #operator_line do
		local char = operator_line:sub(col, col)

		if char ~= " " then
			table.insert(op_positions, col)
		end
	end

	-- using the operator positions split each row into its components
	local result = {}
	for row = 1, height do
		local eq_row = {}

		for i = 1, #op_positions do
			local start_col = op_positions[i]
			local end_col

			if i < #op_positions then
				end_col = op_positions[i + 1] - 2 -- -2 to account for space
			else
				end_col = #lines[row]
			end

			local value = lines[row]:sub(start_col, end_col)
			table.insert(eq_row, value)
		end

		table.insert(result, eq_row)
	end

	local zipped = zip(result)

	for i = 1, #zipped do
		local op = table.remove(zipped[i]):gsub("%s+", "")
		local len = 0

		for j = 1, #zipped[i] do
			local value = zipped[i][j]
			len = math.max(len, #value)
		end

		local new_nums = {}
		for k = 1, len do
			local new_num = ""

			for j = 1, #zipped[i] do
				local cur_len = (len - k) + 1
				local value = string.sub(zipped[i][j], cur_len, cur_len)
				new_num = new_num .. (value or "")
			end

			table.insert(new_nums, tonumber(new_num))
		end

		local answer = reduce(new_nums, op)
		total = total + answer
	end

	print("Part 2: " .. total)
end

part1()
part2()
