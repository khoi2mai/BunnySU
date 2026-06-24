package com.bunny.su.data.repository

import com.bunny.su.data.model.Module
import com.bunny.su.data.model.ModuleUpdateInfo

interface ModuleRepository {
    suspend fun getModules(): Result<List<Module>>
    suspend fun checkUpdate(module: Module): Result<ModuleUpdateInfo>
}
