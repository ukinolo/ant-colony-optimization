import matplotlib.pyplot as plt

num_of_nodes = [8, 12, 16, 20]
seq_avg_times = [1696, 3042, 4736, 6807]
two_thread_time = [981, 1779, 2777, 3983]
four_thread_time = [620, 1100, 1683, 2378]
six_thread_time = [513, 890, 1341, 1910]
eight_thread_time = [488, 803, 1201, 1687]

plt.plot(num_of_nodes, seq_avg_times, marker='o', label="Sequential")
plt.plot(num_of_nodes, two_thread_time, marker='o', label="2 threads")
plt.plot(num_of_nodes, four_thread_time, marker='o', label="4 threads")
plt.plot(num_of_nodes, six_thread_time, marker='o', label="6 threads")
plt.plot(num_of_nodes, eight_thread_time, marker='o', label="8 threads")

plt.xlabel("Number of Nodes")
plt.ylabel("Time [ms]")
plt.title("Performance Comparison")
plt.legend()
plt.grid(True)

plt.savefig("plot.png")
