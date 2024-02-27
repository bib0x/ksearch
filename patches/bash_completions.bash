diff --git a/completions/ksearch.bash b/completions/ksearch.bash
index b0929cc..a465c24 100644
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
+		    COMPREPLY=( $(compgen -W "$files" -- $cur) )
                     return 0
                     ;;
                 --filter)
diff --git a/patches/bash_completions.bash b/patches/bash_completions.bash
index 2ce5a6c..e69de29 100644
--- a/patches/bash_completions.bash
+++ b/patches/bash_completions.bash
@@ -1,20 +0,0 @@
-diff --git a/completions/ksearch.bash b/completions/ksearch.bash
-index ff50ac8..70cc3b1 100644
---- a/completions/ksearch.bash
-+++ b/completions/ksearch.bash
-@@ -33,12 +33,9 @@ _ksearch() {
-                     COMPREPLY=($(compgen -f "${cur}"))
-                     return 0
-                     ;;
--                --topic)
--                    COMPREPLY=($(compgen -f "${cur}"))
--                    return 0
--                    ;;
--                -t)
--                    COMPREPLY=($(compgen -f "${cur}"))
-+                --topic|-t)
-+                    files=$(IFS=:; for path in $KSEARCH_PATH; do find -L $path -type f -name "$cur*.json" -printf '%f\n'; done | sort -u | sed -E 's/\.json$//g')
-+                    COMPREPLY=( $(compgen -W "$files" -- $cur) )
-                     return 0
-                     ;;
-                 --filter)
