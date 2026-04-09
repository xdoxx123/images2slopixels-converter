local function hexbytesintohex(r,g,b)
    local numr = r
    local numg = g
    local numb = b
    local rgb = (numr << 16) | (numg << 8) | numb
    return rgb
end

function parse(data)
    local mainobject = {}
    if string.sub(data,1,6) ~= "SLUDGE" then
        print("no magic number")
        return
    end
    local width = string.byte(data,7)
    print(width)
    mainobject.width = width
    local height = string.byte(data,8)
    print(height)
    mainobject.height = height
    local pixels = {}
    for i = 9,#data,3 do
        print(i)
        local r = string.byte(data,i)
        local g = string.byte(data,i+1)
        local b = string.byte(data,i+2)
        local hex = hexbytesintohex(r,g,b)
        table.insert(pixels,hex)
        
    end

    mainobject.pixels = pixels
        
    
    return mainobject
    
end
