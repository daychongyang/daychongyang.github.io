# React 渲染更新机制

## Hello World!

```js
import React from "react"
import ReactDOM from "react-dom"

class App extends React.PureComponent {
  state = {
    now: new Date().toLocaleTimeString()
  }

  componentDidMount() {
    setInterval(() => {
      this.setState({
        now: new Date().toLocaleTimeString()
      })
    }, 1000)
  }

  render() {
    const { now } = this.state
    return (
      <div>
        <h1>Hello, world!</h1>
        <h2>{now}</h2>
      </div>
    )
  }
}

ReactDOM.render(<App />, document.getElementById("root"))
```

[Babel REPL](https://babeljs.io/repl#?browsers=defaults%2C%20not%20ie%2011%2C%20not%20ie_mob%2011%0A&build=&builtIns=false&spec=false&loose=false&code_lz=JYWwDg9gTgLgBAJQKYEMDG8BmUIjgcilQ3wG4AoUSWRYmAEQHkBZObXAo9GAWgBNcZcuTQAbFAGcJcAIJgwcJAA8YSAHZ9pybgDoACgFciAYVyQ16-AG9ycOBJgpVcALxwbdu2ogB3AFxwFj5w9E5IABQAlDowEAAyEGgookgAKqBIAMowUMBqAOZRADS2cAC-FKVoZhAWagzAfMwQBvVR7qV2EkgwAJL1SFAAbsnh7S4AfB2enjAAFsASOt0w2WHhHjOe3v6BSMGhqlEx8YnJaRnZuQXFnZ5lkRQzZUVwAIwADF-PpWXCdkQNIN2ps7NU1A53Dsyq44PNFstHKonp4iDAjGo4OE7nYADx8YBDCY4zy4uZvCYACSQolEEFePmgoj4AEJcQB6cnErakuYAJgmVmhHP53K2HIJRLuPzsfz-5G0GCYzB0gL4wNxcgU7ImrwEaAMIEsOnyPQAoikjfUAEIAT16fHChAgEBg-EiPyAA&debug=false&forceAllTransforms=false&shippedProposals=false&circleciRepo=&evaluate=false&fileSize=false&timeTravel=false&sourceType=module&lineWrap=false&presets=react%2Ctypescript&prettier=true&targets=&version=7.12.11&externalPlugins=)

[CodeSandbox](https://codesandbox.io/s/suspicious-jones-rbjc6?file=/public/index.html)

```js
const state = reconcile(update)
const UI = commit(state)
```

可触发更新的方法:

1. ReactDOM.render
2. this.setState
3. this.forceUpdate
4. useState
5. useReducer

## ReactDOM.render

[相关源码](https://github.com/facebook/react/blob/6cbb9394d1474e3a728b49dc7f3a11d61a421ce3/packages/react-dom/src/client/ReactDOMLegacy.js#L287)

- 初始挂载 ( `(DOM#root)._reactRootContainer` )

  1.  [创建 fiber](https://github.com/facebook/react/blob/6cbb9394d1474e3a728b49dc7f3a11d61a421ce3/packages/react-dom/src/client/ReactDOMLegacy.js#L193)

      1. [创建 fiberRoot](https://github.com/facebook/react/blob/6cbb9394d1474e3a728b49dc7f3a11d61a421ce3/packages/react-reconciler/src/ReactFiberRoot.old.js#L95)
      2. [创建 rootFiber](https://github.com/facebook/react/blob/6cbb9394d1474e3a728b49dc7f3a11d61a421ce3/packages/react-reconciler/src/ReactFiberRoot.old.js#L102)

  ```js
  fiberRoot.current = rootFiber
  rootFiber.stateNode = fiberRoot

  const container = document.getElementById("root")

  container._reactRootContainer = {
    _internalRoot: fiberRoot
  }

  fiberRoot.containerInfo = container
  ```

  ![react-render](/react/createRootFiber.png)

  2.  [创建 update](https://github.com/facebook/react/blob/6cbb9394d1474e3a728b49dc7f3a11d61a421ce3/packages/react-reconciler/src/ReactFiberReconciler.old.js#L301) `{ payload : { element } }`

  3.  [标记优先级](https://github.com/facebook/react/blob/6cbb9394d1474e3a728b49dc7f3a11d61a421ce3/packages/react-reconciler/src/ReactFiberWorkLoop.old.js#L538)

  4.  [调度更新](https://github.com/facebook/react/blob/6cbb9394d1474e3a728b49dc7f3a11d61a421ce3/packages/react-reconciler/src/ReactFiberWorkLoop.old.js#L646)

  5.  **render**

      - [performSyncWorkOnRoot](https://github.com/facebook/react/blob/6cbb9394d1474e3a728b49dc7f3a11d61a421ce3/packages/react-reconciler/src/ReactFiberWorkLoop.old.js#L614)

        - [renderRootSync](https://github.com/facebook/react/blob/6cbb9394d1474e3a728b49dc7f3a11d61a421ce3/packages/react-reconciler/src/ReactFiberWorkLoop.old.js#L1549)

          - [prepareFreshStack](https://github.com/facebook/react/blob/6cbb9394d1474e3a728b49dc7f3a11d61a421ce3/packages/react-reconciler/src/ReactFiberWorkLoop.old.js#L1557)

            - [createWorkInProgress](https://github.com/facebook/react/blob/6cbb9394d1474e3a728b49dc7f3a11d61a421ce3/packages/react-reconciler/src/ReactFiberWorkLoop.old.js#L1381)

              ```js
              if (rootFiber.alternate === null) {
                workInProgress = createFiber(/** arguments */)
                workInProgress.alternate = rootFiber
                rootFiber.alternate = workInProgress
              }
              ```

              ![workInProgress](/react/workInProgress.png)

          - [workLoopSync](https://github.com/facebook/react/blob/6cbb9394d1474e3a728b49dc7f3a11d61a421ce3/packages/react-reconciler/src/ReactFiberWorkLoop.old.js#L1575)

            ```js
            function workLoopSync() {
              // Already timed out, so perform work without checking if we need to yield.
              while (workInProgress !== null) {
                performUnitOfWork(workInProgress)
              }
            }
            ```

            - [performUnitOfWork](https://github.com/facebook/react/blob/6cbb9394d1474e3a728b49dc7f3a11d61a421ce3/packages/react-reconciler/src/ReactFiberWorkLoop.old.js#L1701) ` workInProgress = workInProgress.child`

              - `递` 自 `rootFiber` 向下深度优先遍历. 为每一个 fiber 节点调用 `beginWork` 方法. 当遍历到叶子节点时, 变会进入当前节点的 "归" 阶段.
              - `归` 为当前 fiber 节点调用 `completeWork` 方法, 且若其存在兄弟节点 `sibling`, 变会进入 `sibling` 节点的 "递" 阶段. 反之, 进入父节点的 "归" 阶段.
              - [beginWork](https://github.com/facebook/react/blob/6cbb9394d1474e3a728b49dc7f3a11d61a421ce3/packages/react-reconciler/src/ReactFiberBeginWork.old.js#L3218) 依据传入的 fiber 节点创建子 fiber 节点, 并连接两个节点.

                - [updateHostRoot `rootFiber`](https://github.com/facebook/react/blob/6cbb9394d1474e3a728b49dc7f3a11d61a421ce3/packages/react-reconciler/src/ReactFiberBeginWork.old.js#L3520)
                  - [update.payload => workInProgress.memoizedState](https://github.com/facebook/react/blob/6cbb9394d1474e3a728b49dc7f3a11d61a421ce3/packages/react-reconciler/src/ReactUpdateQueue.old.js#L571)
                  - [reconcileChildren](https://github.com/facebook/react/blob/6cbb9394d1474e3a728b49dc7f3a11d61a421ce3/packages/react-reconciler/src/ReactFiberBeginWork.old.js#L252) `nextChildren : 👆 update.payload.element`
                    - [reconcileChildFibers](https://github.com/facebook/react/blob/6cbb9394d1474e3a728b49dc7f3a11d61a421ce3/packages/react-reconciler/src/ReactFiberBeginWork.old.js#L276)
                      - [createFiberFromElement](https://github.com/facebook/react/blob/6cbb9394d1474e3a728b49dc7f3a11d61a421ce3/packages/react-reconciler/src/ReactChildFiber.old.js#L1194)
                        - [createFiberFromTypeAndProps](https://github.com/facebook/react/blob/6cbb9394d1474e3a728b49dc7f3a11d61a421ce3/packages/react-reconciler/src/ReactFiber.old.js#L590) `createElement => Fiber`
                - [updateClassComponent `App`](https://github.com/facebook/react/blob/6cbb9394d1474e3a728b49dc7f3a11d61a421ce3/packages/react-reconciler/src/ReactFiberBeginWork.old.js#L3511)

                  - 挂载

                    - [读取 context](https://github.com/facebook/react/blob/6cbb9394d1474e3a728b49dc7f3a11d61a421ce3/packages/react-reconciler/src/ReactFiberClassComponent.old.js#L824)
                    - [构造函数初始化](https://github.com/facebook/react/blob/6cbb9394d1474e3a728b49dc7f3a11d61a421ce3/packages/react-reconciler/src/ReactFiberBeginWork.old.js#L983)
                    - [调用 生命周期函数`getDerivedStateFromProps`](https://github.com/facebook/react/blob/6cbb9394d1474e3a728b49dc7f3a11d61a421ce3/packages/react-reconciler/src/ReactFiberClassComponent.old.js#L866)
                    - [调用 生命周期函数`componentWillMount`](https://github.com/facebook/react/blob/6cbb9394d1474e3a728b49dc7f3a11d61a421ce3/packages/react-reconciler/src/ReactFiberClassComponent.old.js#L885)
                    - [调用 生命周期函数`render`](https://github.com/facebook/react/blob/6cbb9394d1474e3a728b49dc7f3a11d61a421ce3/packages/react-reconciler/src/ReactFiberBeginWork.old.js#L1071)
                      - [reconcileChildren](https://github.com/facebook/react/blob/6cbb9394d1474e3a728b49dc7f3a11d61a421ce3/packages/react-reconciler/src/ReactFiberBeginWork.old.js#L1103)
                        - [mountChildFibers](https://github.com/facebook/react/blob/6cbb9394d1474e3a728b49dc7f3a11d61a421ce3/packages/react-reconciler/src/ReactFiberBeginWork.old.js#L263)
                          - [reconcileChildFibers](https://github.com/facebook/react/blob/6cbb9394d1474e3a728b49dc7f3a11d61a421ce3/packages/react-reconciler/src/ReactChildFiber.old.js#L1240)
                            - [reconcileSingleElement](https://github.com/facebook/react/blob/6cbb9394d1474e3a728b49dc7f3a11d61a421ce3/packages/react-reconciler/src/ReactChildFiber.old.js#L1270)
                              - [createFiberFromElement](https://github.com/facebook/react/blob/6cbb9394d1474e3a728b49dc7f3a11d61a421ce3/packages/react-reconciler/src/ReactChildFiber.old.js#L1194)
                                - [createFiberFromTypeAndProps](https://github.com/facebook/react/blob/6cbb9394d1474e3a728b49dc7f3a11d61a421ce3/packages/react-reconciler/src/ReactFiber.old.js#L590) `createElement => Fiber`

                  - 更新

                    - [调用 生命周期函数`componentWillReceiveProps`](https://github.com/facebook/react/blob/6cbb9394d1474e3a728b49dc7f3a11d61a421ce3/packages/react-reconciler/src/ReactFiberClassComponent.old.js#L866)
                    - [调用 生命周期函数`getDerivedStateFromProps`](https://github.com/facebook/react/blob/6cbb9394d1474e3a728b49dc7f3a11d61a421ce3/packages/react-reconciler/src/ReactFiberClassComponent.old.js#L969)
                    - [调用 生命周期函数`shouldComponentUpdate`](https://github.com/facebook/react/blob/6cbb9394d1474e3a728b49dc7f3a11d61a421ce3/packages/react-reconciler/src/ReactFiberClassComponent.old.js#L866)

                - [updateHostComponent](https://github.com/facebook/react/blob/6cbb9394d1474e3a728b49dc7f3a11d61a421ce3/packages/react-reconciler/src/ReactFiberBeginWork.old.js#L3522)

                - **...**

              - [completeWork](https://github.com/facebook/react/blob/6cbb9394d1474e3a728b49dc7f3a11d61a421ce3/packages/react-reconciler/src/ReactFiberCompleteWork.old.js#L791)

                - 挂载
                  - 为 fiber 节点生成对应的 DOM 节点
                  - 将子 DOM 节点插入更生层的 DOM 节点
                  - 处理 props, 事件注册, style, innerHtml, children...
                - 更新
                  - 处理 props, 事件注册, style, innerHtml, children...
                - [生成 `effectList` 单向链表](https://github.com/facebook/react/blob/6cbb9394d1474e3a728b49dc7f3a11d61a421ce3/packages/react-reconciler/src/ReactFiberWorkLoop.old.js#L1771)
                  - rootFiber.firstEffect (nextEffect)
                  - rootFiber.lastEffect

              ![react-render](/react/react-fiber.png)

        - [commitRoot](https://github.com/facebook/react/blob/6cbb9394d1474e3a728b49dc7f3a11d61a421ce3/packages/react-reconciler/src/ReactFiberWorkLoop.old.js#L1097)

  6.  **commit** 遍历 `effectList`单向链表, 执行副作用(DOM, 生命周期钩子执行)

      - [`DOM 操作前` commitBeforeMutationEffects](https://github.com/facebook/react/blob/6cbb9394d1474e3a728b49dc7f3a11d61a421ce3/packages/react-reconciler/src/ReactFiberWorkLoop.old.js#L2010)
        - [DOM 节点渲染、删除后的 Blur Foucus 事件处理](https://github.com/facebook/react/blob/6cbb9394d1474e3a728b49dc7f3a11d61a421ce3/packages/react-reconciler/src/ReactFiberWorkLoop.old.js#L2266)
        - [commitBeforeMutationLifeCycles](https://github.com/facebook/react/blob/6cbb9394d1474e3a728b49dc7f3a11d61a421ce3/packages/react-reconciler/src/ReactFiberCommitWork.old.js#L255)
          - [调用生命周期钩子 Classic `getSnapshotBeforeUpdate`](https://github.com/facebook/react/blob/6cbb9394d1474e3a728b49dc7f3a11d61a421ce3/packages/react-reconciler/src/ReactFiberCommitWork.old.js#L301)
        - [调度 useEffect](https://github.com/facebook/react/blob/6cbb9394d1474e3a728b49dc7f3a11d61a421ce3/packages/react-reconciler/src/ReactFiberWorkLoop.old.js#L2293)
      - [`执行 DOM 操作` commitMutationEffects](https://github.com/facebook/react/blob/6cbb9394d1474e3a728b49dc7f3a11d61a421ce3/packages/react-reconciler/src/ReactFiberWorkLoop.old.js#L2053)
        - [重置文本内容](https://github.com/facebook/react/blob/6cbb9394d1474e3a728b49dc7f3a11d61a421ce3/packages/react-reconciler/src/ReactFiberWorkLoop.old.js#L2319)
        - [更新 Ref](https://github.com/facebook/react/blob/6cbb9394d1474e3a728b49dc7f3a11d61a421ce3/packages/react-reconciler/src/ReactFiberWorkLoop.old.js#L2322)
        - [DOM 操作](https://github.com/facebook/react/blob/6cbb9394d1474e3a728b49dc7f3a11d61a421ce3/packages/react-reconciler/src/ReactFiberWorkLoop.old.js#L2341)
        - [调用 useLayoutEffect 销毁函数](https://github.com/facebook/react/blob/6cbb9394d1474e3a728b49dc7f3a11d61a421ce3/packages/react-reconciler/src/ReactFiberWorkLoop.old.js#L2360)
        - [调用 useEffect 销毁函数](https://github.com/facebook/react/blob/6cbb9394d1474e3a728b49dc7f3a11d61a421ce3/packages/react-reconciler/src/ReactFiberWorkLoop.old.js#L2360)
        - [调用 生命周期函数 `componentWillUnmount`](https://github.com/facebook/react/blob/6cbb9394d1474e3a728b49dc7f3a11d61a421ce3/packages/react-reconciler/src/ReactFiberWorkLoop.old.js#L2360)
      - [`DOM 操作后` commitLayoutEffects](https://github.com/facebook/react/blob/6cbb9394d1474e3a728b49dc7f3a11d61a421ce3/packages/react-reconciler/src/ReactFiberWorkLoop.old.js#L2088)
        - [commitLayoutEffectOnFiber](https://github.com/facebook/react/blob/6cbb9394d1474e3a728b49dc7f3a11d61a421ce3/packages/react-reconciler/src/ReactFiberCommitWork.old.js#L493)
          - Classic 组件
            - [挂载, 调用生命周期钩子 `componentDidMount`](https://github.com/facebook/react/blob/6cbb9394d1474e3a728b49dc7f3a11d61a421ce3/packages/react-reconciler/src/ReactFiberCommitWork.old.js#L566)
            - [更新, 调用生命周期钩子 `componentDidUpdate`](https://github.com/facebook/react/blob/6cbb9394d1474e3a728b49dc7f3a11d61a421ce3/packages/react-reconciler/src/ReactFiberCommitWork.old.js#L616)
          - Functional 组件
            - [调度 useLayoutEffect 回调函数](https://github.com/facebook/react/blob/6cbb9394d1474e3a728b49dc7f3a11d61a421ce3/packages/react-reconciler/src/ReactFiberCommitWork.old.js#L514)
            - [调度 useEffect](https://github.com/facebook/react/blob/6cbb9394d1474e3a728b49dc7f3a11d61a421ce3/packages/react-reconciler/src/ReactFiberWorkLoop.old.js#L2293)
        - [更新 Ref](https://github.com/facebook/react/blob/6cbb9394d1474e3a728b49dc7f3a11d61a421ce3/packages/react-reconciler/src/ReactFiberWorkLoop.old.js#L2441)

- 更新

  1.  [创建 update](https://github.com/facebook/react/blob/6cbb9394d1474e3a728b49dc7f3a11d61a421ce3/packages/react-dom/src/client/ReactDOMLegacy.js#L219)

  2.  [标记优先级](https://github.com/facebook/react/blob/6cbb9394d1474e3a728b49dc7f3a11d61a421ce3/packages/react-reconciler/src/ReactFiberWorkLoop.old.js#L538)

  3.  [调度更新](https://github.com/facebook/react/blob/6cbb9394d1474e3a728b49dc7f3a11d61a421ce3/packages/react-reconciler/src/ReactFiberWorkLoop.old.js#L646)

  4.  **render**

      - [ensureRootIsScheduled](https://github.com/facebook/react/blob/6cbb9394d1474e3a728b49dc7f3a11d61a421ce3/packages/react-reconciler/src/ReactFiberWorkLoop.old.js#L616)

        - **..调度..**
        - [renderRootSync](https://github.com/facebook/react/blob/6cbb9394d1474e3a728b49dc7f3a11d61a421ce3/packages/react-reconciler/src/ReactFiberWorkLoop.old.js#L867)

          - [workLoopSync](https://github.com/facebook/react/blob/6cbb9394d1474e3a728b49dc7f3a11d61a421ce3/packages/react-reconciler/src/ReactFiberWorkLoop.old.js#L1575)
            - **...**

## this.setState

- [`Component.prototype.setState`](https://github.com/facebook/react/blob/6cbb9394d1474e3a728b49dc7f3a11d61a421ce3/packages/react/src/ReactBaseClasses.js#L57)

  - [this.updater.enqueueSetState](https://github.com/facebook/react/blob/6cbb9394d1474e3a728b49dc7f3a11d61a421ce3/packages/react-reconciler/src/ReactFiberClassComponent.old.js#L193)

    - [创建 update](https://github.com/facebook/react/blob/6cbb9394d1474e3a728b49dc7f3a11d61a421ce3/packages/react-reconciler/src/ReactFiberClassComponent.old.js#L196)
    - [调度更新](https://github.com/facebook/react/blob/6cbb9394d1474e3a728b49dc7f3a11d61a421ce3/packages/react-reconciler/src/ReactFiberClassComponent.old.js#L210)

      - [scheduleUpdateOnFiber](https://github.com/facebook/react/blob/6cbb9394d1474e3a728b49dc7f3a11d61a421ce3/packages/react-reconciler/src/ReactFiberClassComponent.old.js#L210)

        - **...**
        - [performSyncWorkOnRoot](https://github.com/facebook/react/blob/6cbb9394d1474e3a728b49dc7f3a11d61a421ce3/packages/react-reconciler/src/ReactFiberWorkLoop.old.js#L614)

1. [使用 Concurrent 模式（实验性）](https://zh-hans.reactjs.org/docs/concurrent-mode-adoption.html)
