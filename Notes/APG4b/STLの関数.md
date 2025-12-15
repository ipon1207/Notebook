## STL とは

C++で用意されている、関数等がまとまっているものを**STL(Standard Template Library)**という

## min 関数

2 つの引数のうち小さい方の値を返す

```cpp
#include <bits/stdc++.h>
using namespace std;

int main() {
    int answer = min(10, 5);

    cout << answer << endl;  // 5が出力される>>
}
```

- 大小比較できる型であれば、min 関数を使うことができる
- 2 つの引数の型は同じである必要がある

## max 関数

2 つの引数のうち大きい方の値を返す

```cpp
#include <bits/stdc++.h>
using namespace std;

int main() {
    int answer = max(10, 5);

    cout << answer << endl;  // 10が出力される
}
```

## swap 関数

2 つの引数の値を入れ替える

```cpp
#include <bits/stdc++.h>
using namespace std;

int main() {
    int a = 10;
    int b = 5;

    swap(a, b);

    cout << a << " " << b << endl;  // "5 10"が出力される
}
```

## reverse 関数

配列を引数にとり、配列の要素の順番を逆にする

```cpp
#include <bits/stdc++.h>
using namespace std;

int main() {
    vector<int> vec = {1, 5, 3};
    reverse(vec.begin(), vec.end());

    for (int i = 0; i < vec.size(); i++) {
        cout << vec.at(i) << endl;
    }
}
```

- `reverse(vec.begin(), vec.end());` のように、配列の最初と最後を指定することで、配列全体を逆順にできる

## sort 関数

配列を要素の小さい順に並び替える

```cpp
#include <bits/stdc++.h>
using namespace std;

int main() {
    vector<int> vec = {2, 5, 2, 1};
    sort(vec.begin(), vec.end());

    for (int i = 0; i < vec.size(); i++) {
        cout << vec.at(i) << endl;
    }
}
```
