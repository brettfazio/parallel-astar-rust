import java.util.*;

public class graph_gen {
     public static void main(String[] args) {

        int n = 4; // nodes

        ArrayList<Integer>[] adjs = new ArrayList[n];
        for (int i = 0; i < n; i++)
            adjs[i] = new ArrayList<>();

        for (int i = 0; i < n - 1; i++) {
            adjs[i].add(random(i+1,n-1));
        }

        int m = 3; // edges (minimum n - 1 edges)

        for (int i = 0; i <= m - n;) {
            int r = (random(0,n-1));
            int l = (random(0,n-1));
            if (adjs[l].contains(r) || adjs[r].contains(l) || l == r) continue;
            i++;
            adjs[l].add(r);
        }

        System.out.printf("%d %d\n", n, m);

        for (int i = 0; i < n; i++) {
            for (int to:adjs[i]) {
                System.out.printf("%d %d %d\n", i, to, random(1, (int)1e4));
            }
        }
    }

    static int random(int min, int max) {
        int range = max - min + 1; 
        return (int)(Math.random() * range) + min; 
    }
}