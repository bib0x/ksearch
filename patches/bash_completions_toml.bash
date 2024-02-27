diff --git a/completions/ksearch.bash b/completions/ksearch.bash
index b0929cc..a7ed8c6 100644
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
+                -t|--topic)
+		    files=$(IFS=:; for path in $KSEARCH_PATH; do find -L $path -type f -name "$cur*.toml" -printf '%f\n'; done | sort -u | sed -E 's/\.toml$//g')
+                    COMPREPLY=($(compgen -W "${files}" -- ${cur}))
                     return 0
                     ;;
                 --filter)
