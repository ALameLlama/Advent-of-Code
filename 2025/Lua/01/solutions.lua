Range = {}
Range.__index = Range

function Range:new(index)
	local obj = {
		index = index or 0,
		length = 100,
		incrementAmount = 1,
		decrementAmount = 1,
	}
	setmetatable(obj, self)
	return obj
end

function Range:next(steps)
	steps = steps or self.incrementAmount
	self.index = (self.index + self.length + steps) % self.length
	return self.index
end

function Range:previous(steps)
	steps = steps or self.decrementAmount
	self.index = (self.index + self.length - steps) % self.length
	return self.index
end

local function part1()
	local r = Range:new(50)
	local number = 0

	for line in io.lines("input.txt") do
		local dir, count = line:match("(%u)(%d+)")

		if dir == "R" then
			r:next(count)
		else
			r:previous(count)
		end

		if r.index == 0 then
			number = number + 1
		end

		-- print(dir, count, r.index)
	end

	print("Part 1: " .. number)
end

local function part2()
	local r = Range:new(50)
	local number = 0

	for line in io.lines("input.txt") do
		local dir, count = line:match("(%u)(%d+)")

		if dir == "R" then
			for i = 1, count do
				r:next()

				if r.index == 0 then
					number = number + 1
				end
			end
		else
			for i = 1, count do
				r:previous()

				if r.index == 0 then
					number = number + 1
				end
			end
		end

		-- print(dir, count, r.index)
	end

	print("Part 2: " .. number)
end

part1()
part2()
