#include <iostream>
#include <vector>
#include <algorithm>

using namespace std;

struct hook {
    bool empty;
    int s;
    int c;
};

// input
int H, S, C, a, b;
vector<int> K;
vector<vector<int>> N, A, B;

void read_input() {
    cin >> S >> C >> H >> a >> b;
    N.resize(S);
    for (int s = 0; s < S; s++) {
        N[s].resize(C);
        for (int c = 0; c < C; c++) cin >> N[s][c];
    }
    K.resize(S);
    for (int s = 0; s < S; s++) cin >> K[s];
    A.resize(S);
    for (int s = 0; s < S; s++) {
        A[s].resize(S);
        for (int t = 0; t < S; t++) cin >> A[s][t];
    }
    B.resize(C);
    for (int c = 0; c < C; c++) {
        B[c].resize(C);
        for (int d = 0; d < C; d++) cin >> B[c][d];
    }
}

void write_output(const vector<pair<int, int> > &ans) {
    cout << ans.size() << '\n';
    for (int i = 0; i < int(ans.size()); i++) {
        if (ans[i].first >= 0) {
            cout << ans[i].first + 1 << ' ' << ans[i].second + 1 << '\n';
        } else {
            cout << ans[i].first << '\n';
        }
    }
}

int add_pos(int pos, int k) {
    pos = (pos + k) % H;

    if(pos < 0){
        return pos + H;
    }
    return pos;
}

vector<pair<int, int>> sample(void) {
    // initialize varibles
    vector<pair<int, int> > ans;
    vector<int> hangers_available = K;

    vector<hook> hanger;
    for(int i = 0; i < H; i++) {
        hook aux = {true, -1, -1};
        hanger.push_back(aux);
    }

    int pos = 0, prev_s = -1, prev_c = -1;

    for (int s = 0; s < S; s++) {
        for (int c = 0; c < C; c++) { // assign item (s, c)
            if (N[s][c] == 0) continue;

            // need this amount of empty hooks
            const int setup = (prev_s == -1 ? 0 : max(A[prev_s][s], B[prev_c][c]));
            for (int i = 0; i < setup; i++) {
                ans.emplace_back(-2, 0);
                pos = add_pos(pos, 1);
            }

            // put (s, c)
            for (int n = 0; n < N[s][c]; ) {
                if (hanger[pos].s != s and hangers_available[s] == 0) {
                    // run out of hangers
                    ans.emplace_back(-2, 0);
                    pos = add_pos(pos, 1);
                } else {
                    ans.emplace_back(s, c);
                    n++;
                    if (hanger[pos].s != s) {
                        if (!hanger[pos].empty) {
                            hangers_available[hanger[pos].s]++;
                        }
                        hangers_available[s]--;
                        hanger[pos].s = s;
                        hanger[pos].c = c;
                    }
                    pos = add_pos(pos, 1);
                }
                prev_s = s;
                prev_c = c;
            }
        }
    }

    return ans;
}

// initialize varibles
vector<pair<int, int> > ans;
vector<int> hangers_available = K;
vector<hook> hanger;

pair<int, int> next_item(int pos) {

        return

vector<pair<int, int>> greedy(void) {
    int count = 0, pos = 0, prev_s = -1, prev_c = -1;

    for(int i = 0; i < H; i++) {
        hook aux = {true, -1, -1};
        hanger.push_back(aux);
    }

    while(count < S * C) {
        // find the best candidate
        (s, c) = next_item(
            if (N[s][c] == 0) continue;

            // need this amount of empty hooks
            const int setup = (prev_s == -1 ? 0 : max(A[prev_s][s], B[prev_c][c]));
            for (int i = 0; i < setup; i++) {
                ans.emplace_back(-2, 0);
                pos = add_pos(pos, 1);
            }

            // put (s, c)
            for (int n = 0; n < N[s][c]; ) {
                if (hanger[pos].s != s and hangers_available[s] == 0) {
                    // run out of hangers
                    ans.emplace_back(-2, 0);
                    pos = add_pos(pos, 1);
                } else {
                    ans.emplace_back(s, c);
                    n++;
                    if (hanger[pos].s != s) {
                        if (!hanger[pos].empty) {
                            hangers_available[hanger[pos].s]++;
                        }
                        hangers_available[s]--;
                        hanger[pos].s = s;
                        hanger[pos].c = c;
                    }
                    pos = add_pos(pos, 1);
                }
                prev_s = s;
                prev_c = c;
            }
        }
    }

    return ans;
}

int main() {
    read_input();

    // write answer
    write_output(sample());
}
