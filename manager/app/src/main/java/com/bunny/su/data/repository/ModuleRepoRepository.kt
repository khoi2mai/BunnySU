package com.bunny.su.data.repository

import com.bunny.su.data.model.RepoModule

interface ModuleRepoRepository {
    suspend fun fetchModules(): Result<List<RepoModule>>
}
