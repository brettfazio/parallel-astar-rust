import java.util.*;
import java.awt.Point;

public class gen_grid {
    public static void main(String[] args) {
        Scanner in = new Scanner(System.in);

        int n = 10;

        int x1 = random(0, n - 1); // start
        int y1 = random(0, n - 1);

        int x2 = random(0, n - 1); // end
        int y2 = random(0, n - 1);

        while (x2 == x1 && y2 == y1) { // check start != end
            x2 = random(0, n - 1);
            y2 = random(0, n - 1);
        }

        char[][] arr = new char[n][n];

        // System.out.printf("%d %d %d %d\n", x1, y1, x2, y2);

        for (int i = 0; i < n; i++) {
            for (int j = 0; j < n; j++) {
                int wall = random(0, 4);
                if (i == x1 && j == y1) {
                    System.out.print("S");
                    arr[i][j] = 'S';
                }
                else if (i == x2 && j == y2) {
                    System.out.print("E");
                    arr[i][j] = 'E';
                }
                else if (wall == 0) {
                    System.out.print("W");
                    arr[i][j] = 'W';
                }
                else {
                    System.out.print(".");
                    arr[i][j] = '.';
                }
            }
            System.out.println();
        }

        System.out.println(canReach(arr, n));
    }

    static boolean canReach(char[][] arr, int n) {
        boolean[][] visited = new boolean[n][n];
        int x1 = -1, y1 = 1, x2 = -1, y2 = -1;
        for (int i = 0; i < n; i++) {
            for (int j = 0; j < n; j++) {
                if (arr[i][j] == 'S') {
                    x1 = i;
                    y1 = j;
                }
                if (arr[i][j] == 'E') {
                    x2 = i;
                    y2 = j;
                }
            }
            Arrays.fill(visited[i], false);
        }

        ArrayDeque<Point> q = new ArrayDeque<>();
        visited[x1][y1] = true;
        q.add(new Point(x1, y1));

        int[] dx = {-1, 1, 0, 0};
        int[] dy = {0, 0, -1, 1};

        while (!q.isEmpty()) {
            Point at = q.poll();
            
            for (int i = 0; i < 4; i++) {
                int tox = at.x + dx[i];
                int toy = at.y + dy[i];
                if (tox >= 0 && tox < n && toy >= 0 && toy < n && (arr[tox][toy] == '.' || arr[tox][toy] == 'E' )) {
                    if (!visited[tox][toy]) {
                        q.add(new Point(tox, toy));
                        visited[tox][toy] = true;
                    }
                }
            }
        }

        return visited[x2][y2];
    }

    static int random(int min, int max) {
        int range = max - min + 1; 
        return (int)(Math.random() * range) + min; 
    }

}