f = open("a.txt","r")
data = f.readlines()
for line in data:
    x = line.split(" ")
    l = len(x)
    val1 = str(x[1])
    val2 = str(x[0])
    s = ""
    for i in range(2,l):
        s += x[i]
        if i!=l-1:
            s += " "
    s = s.replace('\n','')
    if l>2:
        print(f'''{val1}=>println!("Machine -> {val2} ({s})"),''')
    else:
        print(f'''{val1}=>println!("Machine -> {val2}"),''')

f.close()