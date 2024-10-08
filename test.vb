' 定义点赞视频时长（默认随机出12~14秒视频进行点赞）
Dim langmax, langmin
langmax = 14000
langmin = 12000

' 定义视频随机个数
Dim sprndnum
sprndnum = 999
ShowMessage "视频个数: " & sprndnum

Delay 2000
' 获取屏幕分辨率
Dim screenX, screenY
screenX = GetScreenX()
ShowMessage "当前设备的屏幕横向分辨率:" & screenX
Delay 2000
screenY = GetScreenY()
ShowMessage "当前设备的屏幕纵向分辨率:" & screenY
Delay 1000

' 定义纵向滑动区间
Dim x1, y1, x2, y2
x1 = screenX / 2
y1 = screenY * 4 / 5
x2 = screenX / 2
y2 = screenY / 3

ShowMessage "x1:" & screenY 

' 定义点赞区间
Dim dianzanx, dianzany
dianzanx = screenX / 2
dianzany = screenY / 3

' 定义播放总时长
Dim langlang, number
langlang = 0
number = 0

' 主循环，处理视频个数
For i = 1 To sprndnum
   ' 定义浮动漂移范围随机数
   Dim fdrnd, fdrnd1
   fdrnd = Int(((100 - (-100) + 1) * Rnd()) + (-100))
   fdrnd1 = Int(((10 - (-10) + 1) * Rnd()) + (-10))

   ' 点赞时长300~150毫秒
   Dim dzlangrnd
   dzlangrnd = Int(((200 - 100 + 1) * Rnd()) + 100)

   ' 滑动时长1000~600毫秒
   Dim hdlangrnd
   hdlangrnd = Int(((1000 - 600 + 1) * Rnd()) + 300)

   ' 视频时长16000毫秒~6000毫秒
   Dim splangrnd
   splangrnd = Int(((16000 - 6000 + 1) * Rnd()) + 6000)

   ' 随机滑动浮动坐标
   Dim hdrndx1, hdrndy1, hdrndx2, hdrndy2
   hdrndx1 = x1 + fdrnd
   hdrndy1 = y1 + fdrnd
   hdrndx2 = x2 + fdrnd
   hdrndy2 = y2 + fdrnd

   ' 双击点赞坐标
   Dim dzrndx1, dzrndy1, dzrndx2, dzrndy2
   dzrndx1 = dianzanx + fdrnd
   dzrndy1 = dianzany + fdrnd
   dzrndx2 = dzrndx1 + fdrnd1
   dzrndy2 = dzrndy1 + fdrnd1

   ' 已播放视频数量
   number = number + 1
   ShowMessage "已播放视频: " & number
   Delay 1000
   ShowMessage "剩余视频: " & (sprndnum - number)
   Delay 1000
   ' 滑动操作部分
   ShowMessage "执行滑动操作"
   Delay 1000
   Swipe hdrndx1, hdrndy1, hdrndx2, hdrndy2, hdlangrnd
   ShowMessage "滑动坐标: " & hdrndx1 & ", " & hdrndy1 & " -> " & hdrndx2 & ", " & hdrndy2
   Delay 1000
   ' 输出滑动时长信息
   ShowMessage "滑动时长: " & hdlangrnd & " 毫秒"
   Delay 1000

   ' 如果视频时长在点赞范围内，进行点赞
   If splangrnd >= langmin And splangrnd <= langmax Then
      ShowMessage "随机点赞，剩余" & (sprndnum - number) & "个，播放时长" & splangrnd * 2 / 1000 & "秒"
      Delay 1000
      ShowMessage "执行点赞，视频时长: " & splangrnd * 2 / 1000 & "秒"
      Delay 1000
      ' 视频时长
      Delay splangrnd
      ' 点赞动作
      Tap dzrndx1, dzrndy1
      Delay dzlangrnd
      Tap dzrndx2, dzrndy2
      ' 点赞后，双倍播放时长
      Delay splangrnd
   Else
      ShowMessage "剩余" & (sprndnum - number) & "个，播放时长" & splangrnd / 1000 & "秒"
      Delay 1000
      Delay splangrnd
   End If

   langlang = langlang + splangrnd
Next

ShowMessage "已观看时长（秒）: " & langlang / 1000
Delay 1000

' 提示总时长
For i = 1 To 5
    ShowMessage "已观看时长: " & langlang / 1000 & "秒"
    Delay 1000
Next

' 关闭app
KeyPress "Home"
Delay Int((2000 + 1000) * Rnd()) + 1000
KillApp "app_name"