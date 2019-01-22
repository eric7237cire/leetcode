//FRom squark_0_1
#include <vector>
#include <string>
#include <iostream>
#include <algorithm>
#include <queue>
#include <set>
#include <map>
#include <sstream>
#include <iomanip>
#include <cstdio>
#include <cstdlib>
#include <cstring>
#include <cmath>
#include <ctime>
#include <cassert>
using namespace std;
typedef long long ll;
typedef double R;
#define pb push_back
#define mp make_pair
#define fi first
#define se second
#define FOR(i, s, t) for(i = (s); i < (t); i++)
#define RFOR(i, s, t) for(i = (s)-1; i >= (t); i--)

void debug1(){}

template<class T, class ... Tails>
void debug1(const T &v, const Tails& ... tails){
	cout<<" "<<v;
	debug1(tails...);
}

template<class T, class ... Tails>
void debug(const T &v, const Tails& ... tails){
#ifdef LOCAL
	cout<<"Debug: "<<v;
	debug1(tails...);
	cout<<endl;
#endif
}

const R PI = acos(-1);
const int MAXM = 1<<20;
const int MAXN = 51234;
const int P = 1e9+7;

int l, r;
int a[MAXN][6];
vector<int> e[MAXM];
int mat[MAXN];
bool vis[MAXM];

bool dfs(int i){
	vis[i] = true;
	for(int j : e[i]){
		if(mat[j] == -1){
			mat[j] = i;
			return true;
		}
	}
	for(int j : e[i]){
		if(!vis[mat[j]] && dfs(mat[j])){
			mat[j] = i;
			return true;
		}
	}

	return false;
}

int main(){
#ifdef LOCAL
	//freopen("in.txt", "r", stdin);
	//freopen("out.txt", "w", stdout);
#endif
	int T, i0;
	scanf("%d", &T);
	for(i0 = 1; i0 <= T; i0++){
		int n;
		int i, j, k;
		int ans = 0;
		scanf("%d", &n);
		for(i = 0; i < n; i++)
			for(j = 0; j < 6; j++)
				scanf("%d", &a[i][j]);

		for(i = 0; i < MAXM; i++)
			e[i].clear();
		for(i = 0; i < n; i++)
			for(j = 0; j < 6; j++)
				e[a[i][j]].pb(i);

		memset(mat, -1, sizeof mat);
		l = 1;
		r = 1;
		while(r <= 1e6+5){
			for(i = l; i <= r; i++)
				vis[i] = false;
			if(dfs(r)){
				r++;
				ans = max(ans, r-l);
			}
			else{
				for(i = 0; i < n; i++)
					if(mat[i] == l)
						mat[i] = -1;
				l++;
				r = max(l, r);
			}
//			cout<<l<<" "<<r<<endl;
//			for(i = 0; i < n; i++)
//				cout<<mat[i]<<" ";
//			cout<<endl;
		}

		printf("Case #%d: %d\n", i0, ans);
	}
	return 0;
}
