package com.gameaday.ia_get_mobile

import android.app.Service
import android.content.Intent
import android.os.IBinder

class DownloadService : Service() {
    
    override fun onBind(intent: Intent?): IBinder? {
        return null
    }
    
    override fun onStartCommand(intent: Intent?, flags: Int, startId: Int): Int {
        // TODO: Implement download service functionality
        return START_STICKY
    }
}