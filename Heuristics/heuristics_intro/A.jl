function find_max(i, c, s, days_no_contest)
    best = 1
    max = typemin(Int32)
    
    total_penalty = 0
    for k in 1:26
        total_penalty += (c[k] * days_no_contest[k])
    end
    
    for k in 1:26
        penalty = c[k] * days_no_contest[k]
        score = s[k] - total_penalty + penalty
        if score > max
            max = score
            best = k
        end
    end
    
    return best
end



function schedule(D, c, s, stream::IO)
    days_no_contest = fill(1, 26)
    
    for i in 1:D
        day = find_max(i, c, s[i], days_no_contest)
        println(stream, day)
        
        for j in 1:26
            days_no_contest[j] += 1
        end
        
        days_no_contest[day] = 0
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