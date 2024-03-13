[1mdiff --git a/src/components/GameCast.vue b/src/components/GameCast.vue[m
[1mindex 3b67974..dd4b872 100644[m
[1m--- a/src/components/GameCast.vue[m
[1m+++ b/src/components/GameCast.vue[m
[36m@@ -38,10 +38,4 @@[m [mconst items = [[m
   width: 100%;[m
   height: 200px;[m
 }[m
[31m-[m
[31m-.homeAway {[m
[31m-  display: inline-flex;[m
[31m-  width: 33%;[m
[31m-  height: 300px;[m
[31m-}[m
 </style>[m
[1mdiff --git a/src/components/TeamDisplay.vue b/src/components/TeamDisplay.vue[m
[1mindex 76c5dda..9eca5c5 100644[m
[1m--- a/src/components/TeamDisplay.vue[m
[1m+++ b/src/components/TeamDisplay.vue[m
[36m@@ -8,7 +8,7 @@[m
         <template #list="slotProps">[m
           <div class="flex flex-column">[m
             <div v-for="(item, index) in slotProps.items" :key="index">[m
[31m-              <span class="position"> {{ item.positon }} </span> -[m
[32m+[m[32m              <span class="position"> {{ item.position }} </span> -[m[41m[m
               {{ item.name }}[m
             </div>[m
           </div>[m
[1mdiff --git a/src/views/HomeView.vue b/src/views/HomeView.vue[m
[1mindex 6a980af..9e1ac41 100644[m
[1m--- a/src/views/HomeView.vue[m
[1m+++ b/src/views/HomeView.vue[m
[36m@@ -1,8 +1,11 @@[m
 <template>[m
[31m-  <div class="row flex align-items-center justify-content-between">[m
[32m+[m[32m  <div class="row flex align-items-center just-content-center">[m
     <div class="w-full">[m
       <TeamDisplay :players="homePlayers" :name="'Home'" />[m
     </div>[m
[32m+[m[32m    <div class="w-full">[m
[32m+[m[32m      <TeamStats :players="awayPlayers" :name="'Team Stats'" />[m
[32m+[m[32m    </div>[m
     <div class="w-full">[m
       <TeamDisplay :players="awayPlayers" :name="'Away'" />[m
     </div>[m
[36m@@ -16,6 +19,7 @@[m
 <script setup lang="ts">[m
 import GameCast from "@/components/GameCast.vue";[m
 import TeamDisplay from "@/components/TeamDisplay.vue";[m
[32m+[m[32mimport TeamStats from "@/components/TeamStats.vue";[m
 const homePlayers = [[m
   { position: "PG", name: "Player Name" },[m
   { position: "SG", name: "Player Name" },[m
[36m@@ -31,6 +35,14 @@[m [mconst awayPlayers = [[m
   { position: "PF", name: "Player Name" },[m
   { position: "C", name: "Player Name" },[m
 ];[m
[32m+[m
[32m+[m[32mconst teamStats = [[m
[32m+[m[32m  { position: "PG", name: "Player Name" },[m
[32m+[m[32m  { position: "SG", name: "Player Name" },[m
[32m+[m[32m  { position: "SF", name: "Player Name" },[m
[32m+[m[32m  { position: "PF", name: "Player Name" },[m
[32m+[m[32m  { position: "C", name: "Player Name" },[m
[32m+[m[32m];[m
 </script>[m
 [m
 <style scoped>[m
