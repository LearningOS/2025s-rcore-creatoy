### 简答作业：
1. 正确进入 U 态后，程序的特征还应有：使用 S 态特权指令，访问 S 态寄存器后会报错。 请同学们可以自行测试这些内容（运行 三个 bad 测例 (ch2b_bad_*.rs) ）， 描述程序出错行为，同时注意注明你使用的 sbi 及其版本。

    **ANS:**
    > 测试使用的 sbi 为：RustSBI-QEMU Version 0.2.0-alpha.3，三个 bad 测例报错信息分别为：
    > 
    > |测例|出错行为|错误信息|
    > |:---|:---|:---|
    > |ch2b_bad_address|读写非法地址|PageFault in application, bad addr = 0x0, bad instruction = 0x804003a4, kernel killed it.|
    > |ch2b_bad_instructions|在 User 模式下执行 `sret` 指令|IllegalInstruction in application, kernel killed it.|
    > |ch2b_bad_registers|直接访问 CSR 寄存器而非通过通用寄存器|IllegalInstruction in application, kernel killed it.|

2. 深入理解 trap.S 中两个函数 __alltraps 和 __restore 的作用，并回答如下问题:

    2.1 L40：刚进入 __restore 时，sp 代表了什么值。请指出 __restore 的两种使用情景。

    **ANS:**
    > - 刚进入 __restore 时，sp 代表了内核态压入 TrapContext 的栈顶地址。
    > - __restore 的两种使用情景：  
    >   - 在处理 Trap 完成时自动调用 __restore 恢复用户态程序寄存器状态并返回 Supervisor 模式  
    >   - 在任务切换方法 _switch 返回后调用 __restore 恢复用户态程序寄存器状态并返回 Supervisor 模式  
    >


    2.2 L43-L48：这几行汇编代码特殊处理了哪些寄存器？这些寄存器的的值对于进入用户态有何意义？请分别解释。

    ```asm
    ld t0, 32*8(sp)
    ld t1, 33*8(sp)
    ld t2, 2*8(sp)
    csrw sstatus, t0
    csrw sepc, t1
    csrw sscratch, t2
    ```

    **ANS:**
    > - 这几行汇编代码特殊处理了 sstatus, sepc, sscratch 寄存器。
    > - sstatus 寄存器中存储了 Trap 发生前 CPU 的特权级信息，用于恢复用户态程序特权级
    > - sepc 寄存器中存储了 Trap 处理完成后默认执行的下一条指令的地址，用于恢复用户态程序 pc
    > - sscratch 寄存器中存储了发生 Trap 时的用户态程序栈顶地址，用于用户态程序恢复时恢复用户态程序 sp。

3. L50-L56：为何跳过了 x2 和 x4？

    ```asm
    ld x1, 1*8(sp)
    ld x3, 3*8(sp)
    .set n, 5
    .rept 27
    LOAD_GP %n
    .set n, n+1
    .endr
    ```

    **ANS:**
    > - x2 存储了的是用户态程序的栈顶地址(sp)，在 45 和 48 行已经被写入 sscratch 寄存器，为了不破坏当前的 sp 值，暂时跳过了 x2，并在 60 行中将 sscratch 寄存器中暂存的值写入 sp。
    > - x4 是 tp 寄存器，在我们的程序中没有用到，所以跳过了。

4. L60：该指令之后，sp 和 sscratch 中的值分别有什么意义？

    ```asm
    csrrw sp, sscratch, sp
    ```

    **ANS:**
    > - 在这条指令之前 sp 存储了内核栈的栈顶指针，sscratch 中存储了用户态程序栈顶地址，这条指令交换了 sp 和 sscratch 中的值，最终 sp 保存用户态程序栈顶地址，sscratch 暂存了内核栈的栈顶指针。

5. __restore：中发生状态切换在哪一条指令？为何该指令执行之后会进入用户态？

    **ANS:**
    > - 发生状态切换在 L61，该指令为 `sret`，它将当前的特权级切换为 Supervisor/User 模式(取决于当前所处的模式)，并跳转到 sepc 指定的地址。而在 sepc 中存储了用户态程序即将执行的下一条指令的地址，所以该指令执行之后会进入用户态。

6. L13：该指令之后，sp 和 sscratch 中的值分别有什么意义？

    ```asm
    csrrw sp, sscratch, sp
    ```

    **ANS:**
    > - 在这条指令之前 sp 是用户态的栈顶指针，sscratch 中存储了内核栈程序栈顶地址，这条指令交换了 sp 和 sscratch 中的值，最终 sp 中是内核栈顶地址，sscratch 为用户态程序的栈顶指针。

7. 从 U 态进入 S 态是哪一条指令发生的？

    **ANS:**
    > - 从 U 态进入 S 态是在 syscall 函数中调用的 `ecall` 指令发生的。

---

### 荣誉准则

1. 在完成本次实验的过程（含此前学习的过程）中，我曾分别与 以下各位 就（与本次实验相关的）以下方面做过交流，还在代码中对应的位置以注释形式记录了具体的交流对象及内容：

    无

2. 此外，我也参考了 以下资料 ，还在代码中对应的位置以注释形式记录了具体的参考来源及内容：

    无

3. 我独立完成了本次实验除以上方面之外的所有工作，包括代码与文档。 我清楚地知道，从以上方面获得的信息在一定程度上降低了实验难度，可能会影响起评分。

4. 我从未使用过他人的代码，不管是原封不动地复制，还是经过了某些等价转换。 我未曾也不会向他人（含此后各届同学）复制或公开我的实验代码，我有义务妥善保管好它们。 我提交至本实验的评测系统的代码，均无意于破坏或妨碍任何计算机系统的正常运转。 我清楚地知道，以上情况均为本课程纪律所禁止，若违反，对应的实验成绩将按“-100”分计。