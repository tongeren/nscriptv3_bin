
timer = timerinit()
i = 0

for xroutine to 100{

    coroutine xroutine{
        i ++
        if i > 10000{
            break self
        }
    }
}
coroutine "last" {
        i ++
        if i > 10000{
            print(cat("time:",timerdiff(timer)))
            break self
        }
}