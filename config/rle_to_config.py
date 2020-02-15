if __name__ == '__main__':
    rle = input("RLE: ")
    print()
    print('-----')
    print()
    print("[[patterns]]")
    print("start_x = 30")
    print("start_y = 10")
    living_cells = []
    y = 0
    for line_i, line in enumerate(rle.split('$')):
        x = 0
        for cell_i, cell in enumerate(line):
            if cell.isnumeric():
                if len(line) == cell_i + 1:
                    for r in range(int(cell)):
                        y += 1
                    continue

                if line[cell_i + 1] == 'o':
                    for r in range(int(cell)):
                        living_cells.append([x + r, y])
                x += int(cell)
            elif not line[cell_i - 1].isnumeric():
                if cell == 'o':
                    living_cells.append([x, y])
                x += 1
        y += 1
    print(f"living_cells = {living_cells}")

