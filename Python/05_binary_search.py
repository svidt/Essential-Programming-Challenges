def binary_search(list, item):
    low = 0
    high = len(list) - 1
    steps = 0

    while low <= high:
        steps += 1
        mid = (low + high) // 2
        guess = list[mid]
        if guess == item:
            print ("\nFOUND: @ index -> ", mid)
            print ("steps: ", steps)
            return mid
        if guess > item:
            high = mid - 1
            print ("high: ", high)
        else:
            low = mid + 1
            print ("low:  ", low)
    return None
    
my_list = list(range(1, 240000))

binary_search(my_list, 239999)
