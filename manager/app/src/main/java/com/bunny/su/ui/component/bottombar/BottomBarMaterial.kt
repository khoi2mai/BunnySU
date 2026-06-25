package com.bunny.su.ui.component.bottombar

import androidx.compose.foundation.BorderStroke
import androidx.compose.foundation.background
import androidx.compose.foundation.clickable
import androidx.compose.foundation.layout.Arrangement
import androidx.compose.foundation.layout.Box
import androidx.compose.foundation.layout.Row
import androidx.compose.foundation.layout.WindowInsets
import androidx.compose.foundation.layout.WindowInsetsSides
import androidx.compose.foundation.layout.displayCutout
import androidx.compose.foundation.layout.fillMaxSize
import androidx.compose.foundation.layout.fillMaxWidth
import androidx.compose.foundation.layout.only
import androidx.compose.foundation.layout.padding
import androidx.compose.foundation.layout.size
import androidx.compose.foundation.layout.systemBars
import androidx.compose.foundation.layout.union
import androidx.compose.foundation.layout.windowInsetsPadding
import androidx.compose.foundation.shape.RoundedCornerShape
import androidx.compose.material.icons.Icons
import androidx.compose.material.icons.filled.Extension
import androidx.compose.material.icons.filled.Home
import androidx.compose.material.icons.filled.Settings
import androidx.compose.material.icons.filled.Shield
import androidx.compose.material.icons.outlined.Extension
import androidx.compose.material.icons.outlined.Home
import androidx.compose.material.icons.outlined.Settings
import androidx.compose.material.icons.outlined.Shield
import androidx.compose.material3.Icon
import androidx.compose.material3.MaterialTheme
import androidx.compose.material3.Surface
import androidx.compose.runtime.Composable
import androidx.compose.ui.Alignment
import androidx.compose.ui.Modifier
import androidx.compose.ui.draw.clip
import androidx.compose.ui.graphics.Color
import androidx.compose.ui.res.stringResource
import androidx.compose.ui.unit.dp
import com.bunny.su.Natives
import com.bunny.su.R
import com.bunny.su.ui.LocalMainPagerState
import com.bunny.su.ui.util.rootAvailable

@Composable
fun BottomBarMaterial() {
    val isManager = Natives.isManager
    val fullFeatured = isManager && !Natives.requireNewKernel() && rootAvailable()
    val mainPagerState = LocalMainPagerState.current

    if (!fullFeatured) return

    val items = listOf(
        Triple(R.string.home, Icons.Filled.Home, Icons.Outlined.Home),
        Triple(R.string.superuser, Icons.Filled.Shield, Icons.Outlined.Shield),
        Triple(R.string.module, Icons.Filled.Extension, Icons.Outlined.Extension),
        Triple(R.string.settings, Icons.Filled.Settings, Icons.Outlined.Settings)
    )

    val dockShape = RoundedCornerShape(28.dp)
    val selectedShape = RoundedCornerShape(20.dp)

    val dockColor = MaterialTheme.colorScheme.surfaceContainerHigh
    val selectedColor = MaterialTheme.colorScheme.secondaryContainer
    val selectedIconColor = MaterialTheme.colorScheme.onSecondaryContainer
    val unselectedIconColor = MaterialTheme.colorScheme.onSurfaceVariant
    val borderColor = MaterialTheme.colorScheme.outlineVariant.copy(alpha = 0.28f)

    Box(
        modifier = Modifier
            .fillMaxWidth()
            .windowInsetsPadding(
                WindowInsets.systemBars
                    .union(WindowInsets.displayCutout)
                    .only(WindowInsetsSides.Horizontal + WindowInsetsSides.Bottom)
            )
            .padding(bottom = 14.dp),
        contentAlignment = Alignment.Center
    ) {
        Surface(
            shape = dockShape,
            color = dockColor,
            tonalElevation = 8.dp,
            shadowElevation = 18.dp,
            border = BorderStroke(1.dp, borderColor)
        ) {
            Row(
                modifier = Modifier
                    .clip(dockShape)
                    .padding(8.dp),
                horizontalArrangement = Arrangement.spacedBy(8.dp),
                verticalAlignment = Alignment.CenterVertically
            ) {
                items.forEachIndexed { index, (label, selectedIcon, unselectedIcon) ->
                    val selected = mainPagerState.selectedPage == index

                    Box(
                        modifier = Modifier
                            .size(56.dp)
                            .clip(selectedShape)
                            .background(
                                if (selected) selectedColor
                                else Color.Transparent
                            )
                            .clickable {
                                if (!selected) {
                                    mainPagerState.animateToPage(index)
                                }
                            },
                        contentAlignment = Alignment.Center
                    ) {
                        Icon(
                            imageVector = if (selected) selectedIcon else unselectedIcon,
                            contentDescription = stringResource(label),
                            tint = if (selected) selectedIconColor else unselectedIconColor,
                            modifier = Modifier.size(28.dp)
                        )
                    }
                }
            }
        }
    }
}