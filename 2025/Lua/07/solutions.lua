local inspect = require("inspect")

local input = "test.txt"
-- local input = "input.txt"

local function part1()
	local total = 0
	local lines = {}

	for line in io.lines(input) do
		local char_array = {}
		for i = 1, #line do
			table.insert(char_array, string.sub(line, i, i))
		end
		table.insert(lines, char_array)
	end

	for x, line in ipairs(lines) do
		for y, value in ipairs(line) do
			if value == "S" then
				lines[x + 1][y] = "|"
				goto continue
			end

			if x == 1 then
				goto continue
			end

			if value == "^" then
				if lines[x - 1][y] == "|" then
					total = total + 1
					lines[x][y - 1] = "|"
					lines[x][y + 1] = "|"

					goto continue
				end
			end

			if value == "." then
				if lines[x - 1][y] == "|" then
					lines[x][y] = "|"

					goto continue
				end
			end

			::continue::
		end
	end

	-- print(inspect(lines))

	print("Part 1: " .. total)
end

-- TODO: fix this
local function part2()
	local total = 0
	local lines = {}

	for line in io.lines(input) do
		local char_array = {}
		for i = 1, #line do
			table.insert(char_array, string.sub(line, i, i))
		end
		table.insert(lines, char_array)
	end

	for x, line in ipairs(lines) do
		for y, value in ipairs(line) do
			if value == "S" then
				lines[x + 1][y] = "|"
				goto continue
			end

			if x == 1 then
				goto continue
			end

			if value == "^" then
				if lines[x - 1][y] == "|" then
					if lines[x][y - 1] ~= "|" then
						lines[x][y - 1] = "|"
						total = total + 1
					end
					if lines[x][y + 1] ~= "|" then
						lines[x][y + 1] = "|"
						total = total + 1
					end

					goto continue
				end
			end

			if value == "." then
				if lines[x - 1][y] == "|" then
					lines[x][y] = "|"

					goto continue
				end
			end

			::continue::
		end
	end

	print("Part 2: " .. total)
end

part1()
part2()
