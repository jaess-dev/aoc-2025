from matplotlib import patches
import matplotlib.pyplot as plt
import numpy as np

PATH = "day09"


def main():
    with open(PATH, 'r') as file:
        content = file.read()

    tuples: list[tuple[int, int]] = [
        # type: ignore
        tuple(map(int, line.split(","))) for line in content.split("\n")]

    start, end = (
        (16590, 83979),
        (84201, 15908),
    )

    plot_points(tuples, start, end)


def plot_points(
    points: list[tuple[int, int]],
    rect_p1: tuple[int, int] | None = None,
    rect_p2: tuple[int, int] | None = None
) -> None:

    if not points:
        return

    xs, ys = zip(*points)

    plt.figure(figsize=(8, 8))
    plt.style.use('dark_background')

    # Draw the scatter points
    plt.scatter(xs, ys, s=1, c='cyan')

    ax = plt.gca()

    # Draw rectangle if two points provided
    if rect_p1 and rect_p2:
        x1, y1 = rect_p1
        x2, y2 = rect_p2

        # Compute rectangle position & size
        xmin = min(x1, x2)
        ymin = min(y1, y2)
        width = abs(x2 - x1)
        height = abs(y2 - y1)

        rect = patches.Rectangle(
            (xmin, ymin),
            width,
            height,
            linewidth=2,
            edgecolor="red",
            facecolor="none"
        )
        ax.add_patch(rect)

    ax.invert_yaxis()
    ax.set_aspect("equal")
    ax.axis("off")

    plt.savefig("day09_visualized.png")


if __name__ == "__main__":
    main()
