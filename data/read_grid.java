import java.util.*;
import java.awt.Point;

public class read_grid {
    public static void main(String[] args) {
        Scanner in = new Scanner(System.in);

        int n = Integer.parseInt(in.nextLine());
        char[][] arr = new char[n][n];
        for (int i = 0; i < n; i++) 
            arr[i] = in.nextLine().toCharArray();

        System.out.println(canReach(arr, n));

    }

    static int canReach(char[][] arr, int n) {
        int[][] visited = new int[n][n];
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
            Arrays.fill(visited[i], -1);
        }

        ArrayDeque<Point> q = new ArrayDeque<>();
        visited[x1][y1] = 0;
        q.add(new Point(x1, y1));

        int[] dx = {-1, 1, 0, 0};
        int[] dy = {0, 0, -1, 1};

        while (!q.isEmpty()) {
            Point at = q.poll();
            
            for (int i = 0; i < 4; i++) {
                int tox = at.x + dx[i];
                int toy = at.y + dy[i];
                if (tox >= 0 && tox < n && toy >= 0 && toy < n && (arr[tox][toy] == '.' || arr[tox][toy] == 'E' )) {
                    if (visited[tox][toy] == -1) {
                        q.add(new Point(tox, toy));
                        visited[tox][toy] = visited[at.x][at.y] + 1;
                    }
                }
            }
        }

        return visited[x2][y2];
    }

}