func tester(x){
    return cat("got",x," with version:aa")
}

timer = timerinit()
print("Starting test, please wait 7s","bg")
i = 0
di = 0
tocheck = 4 
loop {
    if di > tocheck {
        tester(i)
        di = 0
    }
    else{
        d = "well"
        di ++
    }
    if timerdiff(timer) > 6999{
        break
    }
    i ++
}
result = math i / 7
print(cat("Scripts ran ",i," result per second = ",result),"g")
