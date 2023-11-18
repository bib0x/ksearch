diff --git a/completions/ksearch.bash b/completions/ksearch.bash
index ff50ac8..70cc3b1 100644
--- a/completions/ksearch.bash
+++ b/completions/ksearch.bash
@@ -33,12 +33,9 @@ _ksearch() {
                     COMPREPLY=($(compgen -f "${cur}"))
                     return 0
                     ;;
-                --topic)
-                    COMPREPLY=($(compgen -f "${cur}"))
-                    return 0
-                    ;;
-                -t)
-                    COMPREPLY=($(compgen -f "${cur}"))
+                --topic|-t)
+                    files=$(IFS=:; for path in $KSEARCH_PATH; do find -L $path -type f -name "$cur*.json" -printf '%f\n'; done | sort -u | sed -E 's/\.json$//g')
+                    COMPREPLY=( $(compgen -W "$files" -- $cur) )
                     return 0
                     ;;
                 --filter)
