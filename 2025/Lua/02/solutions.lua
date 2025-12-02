local function part1()
	local total = 0

	for line in io.lines("input.txt") do
		for range in line:gmatch("[^,%s]+") do
			local start, stop = range:match("(%d+)-(%d+)")

			for id = start, stop do
				local len = string.len(id)
				local half = len / 2
				local p1 = string.sub(id, 1, half)
				local p2 = string.sub(id, half + 1, len)

				if p1 == p2 then
					total = total + id
				end
			end
		end
	end

	print("Part 1: " .. total)
end

local function part2()
	local total = 0

	for line in io.lines("input.txt") do
		for range in line:gmatch("[^,%s]+") do
			local start, stop = range:match("(%d+)-(%d+)")

			for id = start, stop do
				-- convert to string to make :rep work nicer, trying to convert rep to int was much slower
				local id = tostring(id)
				local len = string.len(id)

				for size = 1, len / 2 do
					local repeater = id:sub(1, size)
					local reps = len / size
					if repeater:rep(reps) == id then
						total = total + tonumber(id)
						break
					end
				end
			end
		end
	end

	print("Part 2: " .. total)
end

part1()
part2()
