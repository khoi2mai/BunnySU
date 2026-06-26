package com.bunny.su.ui.component.rebootlistpopup

import androidx.compose.material.icons.Icons
import androidx.compose.material.icons.filled.PowerSettingsNew
import androidx.compose.material3.DropdownMenu
import androidx.compose.material3.DropdownMenuItem
import androidx.compose.material3.Icon
import androidx.compose.material3.IconButton
import androidx.compose.material3.Text
import androidx.compose.runtime.Composable
import androidx.compose.runtime.getValue
import androidx.compose.runtime.mutableStateOf
import androidx.compose.runtime.remember
import androidx.compose.runtime.setValue
import androidx.compose.ui.res.stringResource
import com.bunny.su.R
import com.bunny.su.ui.component.KsuIsValid
import com.bunny.su.ui.util.reboot

@Composable
fun RebootDropdownItems(onItemClick: (String) -> Unit) {
    val rebootOptions = listOf(
        Pair(R.string.reboot, ""),
        Pair(R.string.reboot_recovery, "recovery"),
        Pair(R.string.reboot_bootloader, "bootloader"),
        Pair(R.string.reboot_edl, "edl")
    )

    rebootOptions.forEach { (id, reason) ->
        DropdownMenuItem(
            text = { Text("  " + stringResource(id)) },
            onClick = { onItemClick(reason) }
        )
    }
}

@Composable
fun RebootListPopupMaterial() {
    var expanded by remember { mutableStateOf(false) }

    KsuIsValid {
        IconButton(onClick = { expanded = true }) {
            Icon(
                imageVector = Icons.Filled.PowerSettingsNew,
                contentDescription = stringResource(id = R.string.reboot)
            )
        }

        DropdownMenu(
            expanded = expanded,
            onDismissRequest = { expanded = false }
        ) {
            RebootDropdownItems { reason ->
                expanded = false
                reboot(reason)
            }
        }
    }
}