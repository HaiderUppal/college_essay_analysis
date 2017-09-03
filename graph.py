import matplotlib.pyplot as plt

a_file = open('analysis.txt', 'r')
analysis = a_file.readlines()
x = int(analysis[len(analysis) - 1].split(']')[0][1:])
z = range(x)


fig = plt.figure()
fig.suptitle('college essay word frequency', fontsize=14, fontweight='bold')

ax = fig.add_subplot(111)
fig.subplots_adjust(top=0.85)
ax.set_title('analysis graph')

ax.set_xlabel('occurances')
ax.set_ylabel('analysis index')

plt.plot(z, [ int(analysis[x].split(' ')[2]) for x in z])
plt.show()