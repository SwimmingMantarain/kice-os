
#ifdef __cplusplus
extern "C" {
#endif

void main() {
    *(char*)0xb8000 = 'Q';
    return;
}

#ifdef __cplusplus
}
#endif
