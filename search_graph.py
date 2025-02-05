import matplotlib.pyplot as plt

# Input sizes
sizes = [1000, 2000, 3000, 4000]

# Step counts for Linear Search
ls_best = [1, 1, 1, 1]       # Always 1 step (First element found)
ls_avg = [500, 1000, 1500, 2000]  # Approx. half of the elements checked
ls_worst = [1000, 2000, 3000, 4000]  # All elements checked

# Step counts for Binary Search
bs_best = [3, 3, 3, 3]       # Always constant step
bs_avg = [30, 33, 36, 36]    # Logarithmic growth
bs_worst = [30, 33, 36, 36]  # Logarithmic growth

fig, axes = plt.subplots(2, 1, figsize=(8, 10)) 

# Function to plot with labels on lines
def plot_with_labels(ax, x, y, label, color, marker, linestyle):
    ax.plot(x, y, marker=marker, linestyle=linestyle, color=color)
    ax.text(x[-1], y[-1], label, fontsize=12, verticalalignment='bottom', horizontalalignment='right', color=color)

# Plot for Linear Search
plot_with_labels(axes[0], sizes, ls_best, "Best Case: O(1)", "green", "o", "-")
plot_with_labels(axes[0], sizes, ls_avg, "Avg Case: O(N)", "blue", "s", "--")
plot_with_labels(axes[0], sizes, ls_worst, "Worst Case: O(N)", "red", "^", "-.")

axes[0].set_title("Linear Search Time Complexity")
axes[0].set_xlabel("Input Size (N)")
axes[0].set_ylabel("Step Count")
axes[0].grid(True)

# Plot for Binary Search
plot_with_labels(axes[1], sizes, bs_best, "Best Case: O(1)", "green", "o", "-")
plot_with_labels(axes[1], sizes, bs_worst, "Avg / Worst Case: O(log N)", "blue", "^", "-.")

axes[1].set_title("Binary Search Time Complexity")
axes[1].set_xlabel("Input Size (N)")
axes[1].set_ylabel("Step Count")
axes[1].set_ylim(0, max(bs_worst) + 5)
axes[1].grid(True)

plt.tight_layout()
plt.show()

