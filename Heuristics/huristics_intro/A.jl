function schedule(D, c, s, stream::IO)
    
    for i in 1:D
        println(stream, 1)
    end
    
end

function main()
    D = parse(Int32, readline(stdin))
    c = parse.(Int32, split(readline(stdin)))
    s = fill(zeros(Int32, 0), D)
    
    for i in 1:D
        s[i] = parse.(Int32, split(readline(stdin)))
    end
    
    schedule(D, c, s, stdout)
end

main()