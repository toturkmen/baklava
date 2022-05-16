for i=0,10,1 do 
    print(string.rep(" ", 10 - i)..string.rep("*", i * 2 + 1))
end

for i=9,-1,-1 do
    print(string.rep(" ", 10 - i)..string.rep("*", i * 2 + 1))    
end